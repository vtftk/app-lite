use crate::events::EventMessage;
use crate::script::events::ScriptExecuteEvent;
use crate::script::runtime::ScriptExecutorHandle;
use crate::state::app_data::{
    AppData, AppDataStore, BitsAmount, EventConfig, EventOutcome, EventTrigger, ItemConfig,
    MinimumRequireRole, ThrowableConfig, ThrowableData, UserScriptConfig,
};
use crate::twitch::manager::{
    TwitchEvent, TwitchEventChatMsg, TwitchEventCheerBits, TwitchEventFollow, TwitchEventGiftSub,
    TwitchEventReSub, TwitchEventRedeem, TwitchEventSub, TwitchEventUser, TwitchManager,
};
use anyhow::Context;
use futures::future::BoxFuture;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use log::{debug, error};
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::{broadcast, RwLock};
use tokio::time::Instant;
use uuid::Uuid;

#[derive(Default)]
pub struct EventsState {
    // Last execution time per event
    pub event_last_execution: HashMap<Uuid, Instant>,
}

#[derive(Default, Clone)]
pub struct EventsStateShared {
    inner: Arc<RwLock<EventsState>>,
}

impl EventsStateShared {
    pub async fn is_cooldown_elapsed(&self, uuid: &Uuid, cooldown: Duration) -> bool {
        let now = Instant::now();

        let inner = &*self.inner.read().await;
        if let Some(last_instant) = inner.event_last_execution.get(uuid) {
            let elapsed = now.duration_since(*last_instant);
            elapsed > cooldown
        } else {
            true
        }
    }

    pub async fn set_last_executed(&self, uuid: &Uuid) {
        let now = Instant::now();
        let inner = &mut *self.inner.write().await;
        inner.event_last_execution.insert(*uuid, now);
    }
}

pub fn get_scripts_by_event(app_data: &AppData, name: &str) -> Vec<UserScriptConfig> {
    app_data
        .scripts
        .iter()
        .filter(|script| script.events.iter().any(|event| name.eq(event)))
        .cloned()
        .collect()
}

pub fn execute_scripts(
    script_handle: ScriptExecutorHandle,
    scripts: Vec<UserScriptConfig>,
    event: ScriptExecuteEvent,
) {
    // Spawn task to poll the execute futures
    tokio::spawn(async move {
        // Create futures to execute for each config
        let mut futures = scripts
            .into_iter()
            .map(|script_config| -> BoxFuture<'_, anyhow::Result<()>> {
                Box::pin(script_handle.execute(script_config.script, event.clone()))
            })
            .collect::<FuturesUnordered<BoxFuture<'_, anyhow::Result<()>>>>();

        while let Some(value) = futures.next().await {
            if let Err(err) = value {
                error!("failed to execute script: {:?}", err);
            }
        }
    });
}

pub async fn handle_twitch_events(
    app_data_store: AppDataStore,
    twitch_manager: Arc<TwitchManager>,
    mut twitch_event_rx: broadcast::Receiver<TwitchEvent>,
    event_sender: broadcast::Sender<EventMessage>,
    script_handle: ScriptExecutorHandle,
) {
    let events_state = EventsStateShared::default();

    while let Ok(event) = twitch_event_rx.recv().await {
        debug!("twitch event received: {:?}", event);

        let mut data = {
            let app_data = &*app_data_store.read().await;
            match event {
                // Handle trigger events
                TwitchEvent::Redeem(event) => get_redeem_event_data(app_data, event),
                TwitchEvent::CheerBits(event) => get_cheer_bits_event_data(app_data, event),
                TwitchEvent::Follow(event) => get_follow_event_data(app_data, event),
                TwitchEvent::Sub(event) => get_sub_event_data(app_data, event),
                TwitchEvent::GiftSub(event) => get_gift_sub_event_data(app_data, event),
                TwitchEvent::ResubMsg(event) => get_resub_event_data(app_data, event),
                TwitchEvent::ChatMsg(event) => {
                    let scripts = get_scripts_by_event(app_data, "chat");

                    if !scripts.is_empty() {
                        execute_scripts(
                            script_handle.clone(),
                            scripts,
                            ScriptExecuteEvent::Chat {
                                message: event.message.text.clone(),
                                user_id: event.user_id.clone(),
                                user_name: event.user_name.clone(),
                                user_display_name: event.user_display_name.clone(),
                            },
                        );
                    }

                    get_chat_event_data(app_data, event)
                }

                // Handle change events from websockets
                TwitchEvent::ModeratorsChanged => {
                    let twitch_manager = twitch_manager.clone();
                    tokio::spawn(async move {
                        debug!("reloading mods list");
                        _ = twitch_manager.load_moderator_list().await;
                    });

                    continue;
                }
                TwitchEvent::VipsChanged => {
                    let twitch_manager = twitch_manager.clone();
                    tokio::spawn(async move {
                        debug!("reloading vips list");
                        _ = twitch_manager.load_vip_list().await;
                    });

                    continue;
                }
                TwitchEvent::RewardsChanged => {
                    let twitch_manager = twitch_manager.clone();
                    tokio::spawn(async move {
                        debug!("reloading rewards list");
                        _ = twitch_manager.load_rewards_list().await;
                    });

                    continue;
                }
            }
        };

        // Remove any events that aren't enabled
        data.event_configs
            .retain(|event_config| event_config.enabled);

        // Skip expensive cloning when no events
        if data.event_configs.is_empty() {
            continue;
        }

        // Create futures to execute for each config
        let mut futures = data
            .event_configs
            .into_iter()
            .map(|event_config| -> BoxFuture<'static, ()> {
                Box::pin(process_event_config(
                    app_data_store.clone(),
                    twitch_manager.clone(),
                    event_sender.clone(),
                    events_state.clone(),
                    data.event_data.clone(),
                    event_config,
                ))
            })
            .collect::<FuturesUnordered<BoxFuture<'static, ()>>>();

        // Spawn task to poll the execute futures
        tokio::spawn(async move { while (futures.next().await).is_some() {} });
    }
}

async fn process_event_config(
    app_data_store: AppDataStore,
    twitch_manager: Arc<TwitchManager>,
    event_sender: broadcast::Sender<EventMessage>,
    events_state: EventsStateShared,
    event_data: EventData,
    event_config: EventConfig,
) {
    // Ensure required role is present
    if !assert_required_role(
        &twitch_manager,
        &event_data.user,
        &event_config.require_role,
    )
    .await
    {
        debug!("skipping event: missing required role");
        return;
    }

    let id = event_config.id;

    // Ensure cooldown is not active
    if !events_state
        .is_cooldown_elapsed(&id, Duration::from_millis(event_config.cooldown as u64))
        .await
    {
        debug!("skipping event: cooldown");
        return;
    }

    // Wait for the delay to complete
    let delay = Duration::from_millis(event_config.outcome_delay as u64);
    tokio::time::sleep(delay).await;

    // Read the current app data and execute
    let app_data = &*app_data_store.read().await;
    match get_outcome_event_message(app_data, event_config, event_data) {
        Ok(msg) => {
            _ = event_sender.send(msg);
            events_state.set_last_executed(&id).await;
        }
        Err(err) => {
            error!("failed to produce event outcome:\n{err:?}");
        }
    };
}

#[derive(Debug)]
pub struct EventHandleData {
    event_configs: Vec<EventConfig>,
    event_data: EventData,
}

fn get_redeem_event_data(app_data: &AppData, event: TwitchEventRedeem) -> EventHandleData {
    let event_reward_id = event.reward.id.to_string();
    let event_configs: Vec<EventConfig> = app_data
        .events
        .iter()
        .filter(|event_config| {
            matches!(&event_config.trigger, EventTrigger::Redeem { reward_id } if event_reward_id.eq(reward_id))
        })
        .cloned()
        .collect();

    let event_data = EventData {
        input: None,
        user: Some(TwitchEventUser {
            user_id: event.user_id,
            user_name: event.user_name,
            user_display_name: event.user_display_name,
        }),
    };

    EventHandleData {
        event_configs,
        event_data,
    }
}

fn get_cheer_bits_event_data(app_data: &AppData, event: TwitchEventCheerBits) -> EventHandleData {
    let event_configs: Vec<EventConfig> = app_data
        .events
        .iter()
        .filter(|event_config| {
            matches!(&event_config.trigger, EventTrigger::Bits { min_bits } if event.bits >= *min_bits as i64)
        })
        .cloned()
        .collect();

    let user = match (event.user_id, event.user_name, event.user_display_name) {
        (Some(user_id), Some(user_name), Some(user_display_name)) => Some(TwitchEventUser {
            user_id,
            user_name,
            user_display_name,
        }),
        _ => None,
    };

    let event_data = EventData {
        input: Some(event.bits as u32),
        user,
    };

    EventHandleData {
        event_configs,
        event_data,
    }
}

fn get_follow_event_data(app_data: &AppData, event: TwitchEventFollow) -> EventHandleData {
    let event_configs: Vec<EventConfig> = app_data
        .events
        .iter()
        .filter(|event_config| matches!(&event_config.trigger, EventTrigger::Follow))
        .cloned()
        .collect();

    let event_data = EventData {
        input: None,
        user: Some(TwitchEventUser {
            user_id: event.user_id,
            user_name: event.user_name,
            user_display_name: event.user_display_name,
        }),
    };

    EventHandleData {
        event_configs,
        event_data,
    }
}

fn get_sub_event_data(app_data: &AppData, event: TwitchEventSub) -> EventHandleData {
    let event_configs: Vec<EventConfig> = app_data
        .events
        .iter()
        .filter(|event_config| matches!(&event_config.trigger, EventTrigger::Subscription))
        .cloned()
        .collect();

    let event_data = EventData {
        input: None,
        user: Some(TwitchEventUser {
            user_id: event.user_id,
            user_name: event.user_name,
            user_display_name: event.user_display_name,
        }),
    };

    EventHandleData {
        event_configs,
        event_data,
    }
}

fn get_gift_sub_event_data(app_data: &AppData, event: TwitchEventGiftSub) -> EventHandleData {
    let event_configs: Vec<EventConfig> = app_data
        .events
        .iter()
        .filter(|event_config| matches!(&event_config.trigger, EventTrigger::GiftedSubscription))
        .cloned()
        .collect();

    let user = match (event.user_id, event.user_name, event.user_display_name) {
        (Some(user_id), Some(user_name), Some(user_display_name)) => Some(TwitchEventUser {
            user_id,
            user_name,
            user_display_name,
        }),
        _ => None,
    };

    let event_data = EventData { input: None, user };

    EventHandleData {
        event_configs,
        event_data,
    }
}

fn get_resub_event_data(app_data: &AppData, event: TwitchEventReSub) -> EventHandleData {
    let event_configs: Vec<EventConfig> = app_data
        .events
        .iter()
        .filter(|event_config| matches!(&event_config.trigger, EventTrigger::Subscription))
        .cloned()
        .collect();

    let event_data = EventData {
        input: None,
        user: Some(TwitchEventUser {
            user_id: event.user_id,
            user_name: event.user_name,
            user_display_name: event.user_display_name,
        }),
    };

    EventHandleData {
        event_configs,
        event_data,
    }
}

fn get_chat_event_data(app_data: &AppData, event: TwitchEventChatMsg) -> EventHandleData {
    let event_configs: Vec<EventConfig> = app_data
        .events
        .iter()
        .filter(|event_config| match &event_config.trigger {
            EventTrigger::Command { message } => {
                let left = message.trim().to_lowercase();
                let right = event
                    .message
                    .fragments
                    .first()
                    .map(|frag| frag.text())
                    .unwrap_or_default()
                    .trim()
                    .to_lowercase();

                left == right
            }
            _ => false,
        })
        .cloned()
        .collect();

    let event_data = EventData {
        input: None,
        user: Some(TwitchEventUser {
            user_id: event.user_id,
            user_name: event.user_name,
            user_display_name: event.user_display_name,
        }),
    };

    EventHandleData {
        event_configs,
        event_data,
    }
}

#[derive(Debug, Clone)]
pub struct EventData {
    /// Represents the data provided by the trigger, i.e amount of bits
    /// total number of subs, number of raiders etc
    input: Option<u32>,

    /// User who triggered the event
    user: Option<TwitchEventUser>,
}

async fn assert_required_role(
    twitch_manager: &TwitchManager,
    user: &Option<TwitchEventUser>,
    required_role: &MinimumRequireRole,
) -> bool {
    match required_role {
        MinimumRequireRole::None => true,
        MinimumRequireRole::Vip => {
            let user = match user {
                Some(user) => user,
                None => return false,
            };

            let vips = match twitch_manager.get_vip_list().await {
                Ok(value) => value,
                Err(_) => {
                    return false;
                }
            };

            vips.iter().any(|vip| vip.user_id == user.user_id)
        }
        MinimumRequireRole::Mod => {
            let user = match user {
                Some(user) => user,
                None => return false,
            };

            let mods = match twitch_manager.get_moderator_list().await {
                Ok(value) => value,
                Err(_) => {
                    return false;
                }
            };

            mods.iter().any(|mods| mods.user_id == user.user_id)
        }
    }
}

fn get_outcome_event_message(
    app_data: &AppData,
    event_config: EventConfig,
    event_data: EventData,
) -> anyhow::Result<EventMessage> {
    match event_config.outcome {
        EventOutcome::ThrowBits(data) => {
            let input = event_data.input.context("throw bits missing input")?;
            let sets = [data._1, data._100, data._1000, data._5000, data._10000];
            let mut bit_index: usize = match input {
                1..=99 => 0,
                100..=999 => 1,
                1000..=4999 => 2,
                5000..=9999 => 3,
                _ => 4,
            };

            let mut bit_icon: Option<Uuid> = None;

            // Go through the bit icons till we find one
            while bit_icon.is_none() {
                bit_icon = sets.get(bit_index).and_then(|value| *value);

                // Move to index before
                match bit_index.checked_sub(1) {
                    Some(value) => {
                        bit_index = value;
                    }
                    None => break,
                }
            }

            let bit_icon = bit_icon.context("no bit icon available")?;

            let item = app_data
                .items
                .iter()
                .find(|item| bit_icon.eq(&item.id))
                .cloned()
                .context("bit icon item missing")?;

            let throwable_config = create_throwable_config(vec![item], app_data);

            let amount = match data.amount {
                BitsAmount::Dynamic { max_amount } => input.min(max_amount),
                BitsAmount::Fixed { amount } => amount,
            };

            Ok(EventMessage::ThrowItem {
                config: throwable_config,
                amount,
            })
        }
        EventOutcome::Throwable(data) => match data.data {
            ThrowableData::Throw {
                throwable_ids,
                amount,
            } => {
                let item = app_data
                    .items
                    .iter()
                    .find(|item| throwable_ids.contains(&item.id))
                    .cloned()
                    .context("throwable item not found")?;

                let throwable_config = create_throwable_config(vec![item], app_data);

                Ok(EventMessage::ThrowItem {
                    config: throwable_config,
                    amount,
                })
            }
            ThrowableData::Barrage {
                throwable_ids,
                amount_per_throw,
                frequency,
                amount,
            } => {
                let items: Vec<ItemConfig> = app_data
                    .items
                    .iter()
                    .filter(|item| throwable_ids.contains(&item.id))
                    .cloned()
                    .collect();

                let throwable_config = create_throwable_config(items, app_data);

                Ok(EventMessage::ThrowItemBarrage {
                    config: throwable_config,
                    amount,
                    frequency,
                    amount_per_throw,
                })
            }
        },

        EventOutcome::TriggerHotkey(data) => Ok(EventMessage::TriggerHotkey {
            hotkey_id: data.hotkey_id,
        }),
        EventOutcome::PlaySound(data) => {
            let config = app_data
                .sounds
                .iter()
                .find(|item| item.id == data.sound_id)
                .cloned()
                .context("sound config not found")?;

            Ok(EventMessage::PlaySound { config })
        }
    }
}

fn create_throwable_config(items: Vec<ItemConfig>, app_data: &AppData) -> ThrowableConfig {
    // Find all the referenced sounds
    let impact_sounds = app_data
        .sounds
        .iter()
        .filter(|sound| {
            items
                .iter()
                .any(|item| item.impact_sounds_ids.contains(&sound.id))
        })
        .cloned()
        .collect();

    ThrowableConfig {
        items,
        impact_sounds,
    }
}
