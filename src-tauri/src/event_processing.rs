use crate::events::EventMessage;
use crate::state::app_data::{
    AppData, AppDataStore, BitsAmount, EventConfig, EventOutcome, EventTrigger, ItemConfig,
    MinimumRequireRole, ThrowableConfig, ThrowableData,
};
use crate::twitch::manager::{
    TwitchEvent, TwitchEventChatMsg, TwitchEventCheerBits, TwitchEventFollow, TwitchEventGiftSub,
    TwitchEventReSub, TwitchEventRedeem, TwitchEventSub, TwitchEventUser, TwitchManager,
};
use log::debug;
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
            if elapsed > cooldown {
                return true;
            }
        }

        false
    }

    pub async fn set_last_executed(&self, uuid: &Uuid) {
        let now = Instant::now();
        let inner = &mut *self.inner.write().await;
        inner.event_last_execution.insert(*uuid, now);
    }
}

pub struct HandleEventData {
    app_data: Arc<AppData>,
    twitch_manager: Arc<TwitchManager>,
    event_sender: broadcast::Sender<EventMessage>,
}

pub async fn handle_twitch_events(
    app_data: AppDataStore,
    twitch_manager: Arc<TwitchManager>,
    mut twitch_event_rx: broadcast::Receiver<TwitchEvent>,
    event_sender: broadcast::Sender<EventMessage>,
) {
    let events_state = EventsStateShared::default();

    while let Ok(event) = twitch_event_rx.recv().await {
        let app_data = &*app_data.read().await;

        debug!("twitch event received: {:?}", event);

        match event {
            TwitchEvent::Redeem(event) => {
                handle_redeem_event(app_data, &twitch_manager, &event_sender, event)
            }
            TwitchEvent::CheerBits(event) => {
                handle_cheer_bits_event(app_data, &twitch_manager, &event_sender, event)
            }
            TwitchEvent::Follow(event) => {
                handle_follow_event(app_data, &twitch_manager, &event_sender, event)
            }
            TwitchEvent::Sub(event) => {
                handle_sub_event(app_data, &twitch_manager, &event_sender, event)
            }
            TwitchEvent::GiftSub(event) => {
                handle_gift_sub_event(app_data, &twitch_manager, &event_sender, event)
            }
            TwitchEvent::ResubMsg(event) => {
                handle_resub_event(app_data, &twitch_manager, &event_sender, event)
            }
            TwitchEvent::ChatMsg(event) => {
                handle_chat_msg_event(app_data, &twitch_manager, &event_sender, event)
            }
            TwitchEvent::ModeratorsChanged => {
                let twitch_manager = twitch_manager.clone();
                tokio::spawn(async move {
                    twitch_manager.reload_moderator_list().await;
                });
            }
            TwitchEvent::VipsChanged => {
                let twitch_manager = twitch_manager.clone();
                tokio::spawn(async move {
                    twitch_manager.reload_vip_list().await;
                });
            }
            TwitchEvent::RewardsChanged => {
                let twitch_manager = twitch_manager.clone();
                tokio::spawn(async move {
                    twitch_manager.reload_rewards_list().await;
                });
            }
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

fn handle_redeem_event(
    app_data: &AppData,
    twitch_manager: &Arc<TwitchManager>,
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventRedeem,
) {
    debug!("twitch redeem event received: {:?}", event);

    let app_data = Arc::new(app_data.clone());

    for event_config in &app_data.events {
        let event_reward_id = event.reward.id.to_string();
        // Filter out events  that don't match
        match &event_config.trigger {
            EventTrigger::Redeem { reward_id } => {
                debug!("checking reward {} {}", event_reward_id, reward_id);
                if event_reward_id.ne(reward_id) {
                    continue;
                }
            }
            _ => continue,
        }

        tokio::spawn(execute_event_config(
            app_data.clone(),
            twitch_manager.clone(),
            event_config.clone(),
            event_sender.clone(),
            EventData {
                input: None,
                user: Some(TwitchEventUser {
                    user_id: event.user_id.clone(),
                    user_name: event.user_name.clone(),
                    user_display_name: event.user_display_name.to_string().into(),
                }),
            },
        ));
    }
}

fn handle_cheer_bits_event(
    app_data: &AppData,
    twitch_manager: &Arc<TwitchManager>,
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventCheerBits,
) {
    let app_data = Arc::new(app_data.clone());
    for event_config in &app_data.events {
        match &event_config.trigger {
            EventTrigger::Bits { min_bits } => {
                if event.bits < *min_bits as i64 {
                    continue;
                }
            }
            _ => continue,
        };

        let user = match (
            event.user_id.as_ref(),
            event.user_name.as_ref(),
            event.user_display_name.as_ref(),
        ) {
            (Some(user_id), Some(user_name), Some(user_display_name)) => Some(TwitchEventUser {
                user_id: user_id.clone(),
                user_name: user_name.to_string().into(),
                user_display_name: user_display_name.to_string().into(),
            }),
            _ => None,
        };

        // TODO: TRIGGER
        tokio::spawn(execute_event_config(
            app_data.clone(),
            twitch_manager.clone(),
            event_config.clone(),
            event_sender.clone(),
            EventData {
                input: Some(event.bits as u32),
                user,
            },
        ));
    }
}

fn handle_follow_event(
    app_data: &AppData,
    twitch_manager: &Arc<TwitchManager>,
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventFollow,
) {
    let app_data = Arc::new(app_data.clone());

    for event_config in &app_data.events {
        if !matches!(&event_config.trigger, EventTrigger::Follow) {
            continue;
        }

        tokio::spawn(execute_event_config(
            app_data.clone(),
            twitch_manager.clone(),
            event_config.clone(),
            event_sender.clone(),
            EventData {
                input: None,
                user: Some(TwitchEventUser {
                    user_id: event.user_id.clone(),
                    user_name: event.user_name.clone(),
                    user_display_name: event.user_display_name.to_string().into(),
                }),
            },
        ));
    }
}

fn handle_sub_event(
    app_data: &AppData,
    twitch_manager: &Arc<TwitchManager>,
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventSub,
) {
    let app_data = Arc::new(app_data.clone());
    for event_config in &app_data.events {
        if !matches!(&event_config.trigger, EventTrigger::Subscription) {
            continue;
        }

        tokio::spawn(execute_event_config(
            app_data.clone(),
            twitch_manager.clone(),
            event_config.clone(),
            event_sender.clone(),
            EventData {
                input: None,
                user: Some(TwitchEventUser {
                    user_id: event.user_id.clone(),
                    user_name: event.user_name.clone(),
                    user_display_name: event.user_display_name.to_string().into(),
                }),
            },
        ));
    }
}

fn handle_gift_sub_event(
    app_data: &AppData,
    twitch_manager: &Arc<TwitchManager>,
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventGiftSub,
) {
    let app_data = Arc::new(app_data.clone());
    for event_config in &app_data.events {
        if !matches!(&event_config.trigger, EventTrigger::GiftedSubscription) {
            continue;
        }

        let user = match (
            event.user_id.as_ref(),
            event.user_name.as_ref(),
            event.user_display_name.as_ref(),
        ) {
            (Some(user_id), Some(user_name), Some(user_display_name)) => Some(TwitchEventUser {
                user_id: user_id.clone(),
                user_name: user_name.to_string().into(),
                user_display_name: user_display_name.to_string().into(),
            }),
            _ => None,
        };

        tokio::spawn(execute_event_config(
            app_data.clone(),
            twitch_manager.clone(),
            event_config.clone(),
            event_sender.clone(),
            EventData { input: None, user },
        ));
    }
}

fn handle_resub_event(
    app_data: &AppData,
    twitch_manager: &Arc<TwitchManager>,
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventReSub,
) {
    let app_data = Arc::new(app_data.clone());
    for event_config in &app_data.events {
        if !matches!(&event_config.trigger, EventTrigger::Subscription) {
            continue;
        }

        tokio::spawn(execute_event_config(
            app_data.clone(),
            twitch_manager.clone(),
            event_config.clone(),
            event_sender.clone(),
            EventData {
                input: None,
                user: Some(TwitchEventUser {
                    user_id: event.user_id.clone(),
                    user_name: event.user_name.clone(),
                    user_display_name: event.user_display_name.to_string().into(),
                }),
            },
        ));
    }
}

fn handle_chat_msg_event(
    app_data: &AppData,
    twitch_manager: &Arc<TwitchManager>,
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventChatMsg,
) {
    let app_data = Arc::new(app_data.clone());
    for event_config in &app_data.events {
        match &event_config.trigger {
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

                if left != right {
                    continue;
                }
            }
            _ => continue,
        };

        tokio::spawn(execute_event_config(
            app_data.clone(),
            twitch_manager.clone(),
            event_config.clone(),
            event_sender.clone(),
            EventData {
                input: None,
                user: Some(TwitchEventUser {
                    user_id: event.user_id.clone(),
                    user_name: event.user_name.clone(),
                    user_display_name: event.user_display_name.to_string().into(),
                }),
            },
        ));
    }
}

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

async fn execute_event_config(
    app_data: Arc<AppData>,
    twitch_manager: Arc<TwitchManager>,
    event_config: EventConfig,
    event_sender: broadcast::Sender<EventMessage>,
    event_data: EventData,
) {
    // Skip disabled events
    if !event_config.enabled {
        return;
    }

    // TODO: WAIT FOR COOLDOWN TO COMPLETE
    if !assert_required_role(
        &twitch_manager,
        &event_data.user,
        &event_config.require_role,
    )
    .await
    {
        return;
    }

    debug!("executing event outcome: {:?}", event_config);

    let delay = Duration::from_millis(event_config.outcome_delay as u64);
    tokio::time::sleep(delay).await;

    match event_config.outcome {
        EventOutcome::ThrowBits(data) => {
            let input = match event_data.input {
                Some(value) => value,
                None => return,
            };

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

            let bit_icon = match bit_icon {
                Some(bit_icon) => bit_icon,
                None => return,
            };

            let item = app_data.items.iter().find(|item| bit_icon.eq(&item.id));
            let item = match item {
                Some(value) => value.clone(),
                None => return,
            };

            let throwable_config = create_throwable_config(vec![item], &app_data);

            let amount = match data.amount {
                BitsAmount::Dynamic { max_amount } => input.min(max_amount),
                BitsAmount::Fixed { amount } => amount,
            };

            _ = event_sender.send(EventMessage::ThrowItem {
                config: throwable_config,
                amount,
            });
        }
        EventOutcome::Throwable(data) => match data.data {
            ThrowableData::Throw {
                throwable_ids,
                amount,
            } => {
                let items = app_data
                    .items
                    .iter()
                    .find(|item| throwable_ids.contains(&item.id));

                let item = match items {
                    Some(value) => value.clone(),
                    // Throwable no longer exists
                    None => return,
                };

                let throwable_config = create_throwable_config(vec![item], &app_data);

                _ = event_sender.send(EventMessage::ThrowItem {
                    config: throwable_config,
                    amount,
                });
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

                let throwable_config = create_throwable_config(items, &app_data);

                _ = event_sender.send(EventMessage::ThrowItemBarrage {
                    config: throwable_config,
                    amount,
                    frequency,
                    amount_per_throw,
                });
            }
        },

        EventOutcome::TriggerHotkey(data) => {
            _ = event_sender.send(EventMessage::TriggerHotkey {
                hotkey_id: data.hotkey_id,
            });
        }
        EventOutcome::PlaySound(data) => {
            let sound = app_data.sounds.iter().find(|item| item.id == data.sound_id);
            let config = match sound {
                Some(value) => value.clone(),
                // Throwable no longer exists
                None => return,
            };

            _ = event_sender.send(EventMessage::PlaySound { config });
        }
    }
}
