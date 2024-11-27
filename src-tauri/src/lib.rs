use anyhow::Context;
use events::{EventMessage, EventRecvHandle};
use log::{debug, error};
use state::{
    app_data::{
        AppData, AppDataStore, BitsAmount, EventConfig, EventOutcome, EventTrigger, ItemConfig,
        ThrowableConfig, ThrowableImageConfig,
    },
    runtime_app_data::RuntimeAppDataStore,
};
use std::{sync::Arc, time::Duration};
use tauri::{App, Manager};
use tokio::sync::broadcast;
use twitch::manager::{
    TwitchEvent, TwitchEventChatMsg, TwitchEventCheerBits, TwitchEventFollow, TwitchEventGiftSub,
    TwitchEventReSub, TwitchEventRedeem, TwitchEventSub, TwitchManager,
};
use twitch_api::{
    twitch_oauth2::{AccessToken, UserToken},
    HelixClient,
};
use uuid::Uuid;

mod commands;
mod constants;
mod events;
mod http;
mod state;
mod twitch;

/// Prevent slow changes from macro by using a separate entrypoint
/// from the macro
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .setup(move |app| {
            let handle = app.handle().clone();

            let (twitch_manager, twitch_event_rx) =
                TwitchManager::new(HelixClient::default(), handle.clone());
            let (event_tx, event_rx) = create_event_channel();

            let app_data = tauri::async_runtime::block_on(load_app_data(app))
                .expect("failed to load app data");

            let runtime_app_data = RuntimeAppDataStore::new(handle.clone());

            // Provide app data and runtime app data stores
            app.manage(app_data.clone());
            app.manage(runtime_app_data.clone());

            // Provide access to twitch manager and event sender
            app.manage(event_tx.clone());
            app.manage(twitch_manager.clone());

            // Attempt to authenticate with twitch using the saved token
            _ = tauri::async_runtime::spawn(attempt_twitch_auth_existing_token(
                app_data.clone(),
                twitch_manager.clone(),
            ));

            // Handle events triggered by twitch
            _ = tauri::async_runtime::spawn(handle_twitch_events(
                app_data.clone(),
                twitch_event_rx,
                event_tx.clone(),
            ));

            // Run HTTP server
            _ = tauri::async_runtime::spawn(async move {
                _ = http::server::start(
                    event_rx,
                    handle,
                    twitch_manager,
                    app_data,
                    runtime_app_data,
                )
                .await;
            });

            // TODO: Start server and block until a channel reports back that the server started?
            // store server task in a state variable to allow attempting restart within app
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::auth::get_twitch_oauth_uri,
            commands::auth::is_authenticated,
            commands::auth::open_twitch_oauth_uri,
            commands::calibration::set_calibration_step,
            commands::test::test_throw,
            commands::test::test_throw_barrage,
            commands::test::test_sound,
            commands::data::get_app_data,
            commands::data::get_runtime_app_data,
            commands::data::set_app_data,
            commands::data::upload_file,
            commands::data::update_hotkeys,
            commands::twitch::get_redeems_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_event_channel() -> (broadcast::Sender<EventMessage>, EventRecvHandle) {
    let (event_tx, rx) = broadcast::channel(10);
    let event_rx = EventRecvHandle(rx);

    (event_tx, event_rx)
}

async fn load_app_data(app: &App) -> anyhow::Result<AppDataStore> {
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;
    let app_data_file = app_data_path.join("data.json");

    debug!("app data path: {:?}", app_data_path);

    let app_data = AppDataStore::load(app_data_file)
        .await
        .context("failed to load app data")?;

    Ok(app_data)
}

/// Attempts to authenticate with twitch using an existing access token
async fn attempt_twitch_auth_existing_token(
    app_data_store: AppDataStore,
    twitch_manager: Arc<TwitchManager>,
) {
    let app_data = app_data_store.read().await;

    let access_token = match app_data.twitch_config.access_token.as_ref() {
        Some(value) => value,
        None => return,
    };

    let access_token = AccessToken::from(access_token.as_str());

    // Create user token (Validates it with the twitch backend)
    let user_token = match UserToken::from_existing(
        &twitch_manager.helix_client,
        access_token,
        None,
        None,
    )
    .await
    {
        Ok(value) => value,
        Err(err) => {
            error!("stored access token is invalid: {}", err);

            // Drop read access to app data
            drop(app_data);

            // Clear twitch token
            _ = app_data_store
                .write(|app_data| {
                    app_data.twitch_config.access_token = None;
                })
                .await;

            return;
        }
    };

    twitch_manager.set_authenticated(user_token).await;
}

async fn handle_twitch_events(
    app_data: AppDataStore,
    mut twitch_event_rx: broadcast::Receiver<TwitchEvent>,
    event_sender: broadcast::Sender<EventMessage>,
) {
    while let Ok(event) = twitch_event_rx.recv().await {
        let app_data = &*app_data.read().await;
        let events = &app_data.events;

        debug!("twitch event received: {:?}", event);

        match event {
            TwitchEvent::Redeem(event) => handle_redeem_event(app_data, &event_sender, event),
            TwitchEvent::CheerBits(event) => handle_cheer_bits_event(events, &event_sender, event),
            TwitchEvent::Follow(event) => handle_follow_event(events, &event_sender, event),
            TwitchEvent::Sub(event) => handle_sub_event(events, &event_sender, event),
            TwitchEvent::GiftSub(event) => handle_gift_sub_event(events, &event_sender, event),
            TwitchEvent::ResubMsg(event) => handle_resub_event(events, &event_sender, event),
            TwitchEvent::ChatMsg(event) => handle_chat_msg_event(events, &event_sender, event),
        }
    }
}

fn handle_redeem_event(
    app_data: &AppData,
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

        // TODO: TRIGGER
        execute_event_config(
            app_data.clone(),
            event_config.clone(),
            event_sender.clone(),
            None,
        );
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

///
///
/// `input` represents the data provided by the trigger, i.e amount of bits
/// total number of subs, number of raiders etc
fn execute_event_config(
    app_data: Arc<AppData>,
    event_config: EventConfig,
    event_sender: broadcast::Sender<EventMessage>,
    input: Option<u32>,
) {
    // Skip disabled events
    if !event_config.enabled {
        return;
    }

    // TODO: WAIT FOR COOLDOWN TO COMPLETE
    // TODO: CHECK USER HAS REQUIRED ROLE

    debug!("executing event outcome: {:?}", event_config);

    let delay = Duration::from_millis(event_config.outcome_delay as u64);

    match event_config.outcome {
        EventOutcome::ThrowBits(data) => {
            let input = match input {
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

            tokio::spawn(async move {
                tokio::time::sleep(delay).await;

                _ = event_sender.send(EventMessage::ThrowItem {
                    config: throwable_config,
                    amount,
                });
            });
        }
        EventOutcome::Throwable(data) => match data.data {
            state::app_data::ThrowableData::Throw {
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

                tokio::spawn(async move {
                    tokio::time::sleep(delay).await;

                    _ = event_sender.send(EventMessage::ThrowItem {
                        config: throwable_config,
                        amount,
                    });
                });
            }
            state::app_data::ThrowableData::Barrage {
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

                tokio::spawn(async move {
                    tokio::time::sleep(delay).await;

                    _ = event_sender.send(EventMessage::ThrowItemBarrage {
                        config: throwable_config,
                        amount,
                        frequency,
                        amount_per_throw,
                    });
                });
            }
        },

        EventOutcome::TriggerHotkey(data) => {
            tokio::spawn(async move {
                tokio::time::sleep(delay).await;
                _ = event_sender.send(EventMessage::TriggerHotkey {
                    hotkey_id: data.hotkey_id,
                });
            });
        }
        EventOutcome::PlaySound(data) => {
            let sound = app_data.sounds.iter().find(|item| item.id == data.sound_id);
            let sound = match sound {
                Some(value) => value,
                // Throwable no longer exists
                None => return,
            };
            let config = sound.clone();

            tokio::spawn(async move {
                tokio::time::sleep(delay).await;
                _ = event_sender.send(EventMessage::PlaySound { config });
            });
        }
    }
}

fn handle_cheer_bits_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventCheerBits,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Bits { min_bits } => {}
            _ => {}
        };
    }
}

fn handle_follow_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventFollow,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Follow => {}
            _ => {}
        };
    }
}

fn handle_sub_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventSub,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Subscription => {}
            _ => {}
        };
    }
}

fn handle_gift_sub_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventGiftSub,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::GiftedSubscription => {}
            _ => {}
        };
    }
}

fn handle_resub_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventReSub,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Subscription => {}
            _ => {}
        };
    }
}

fn handle_chat_msg_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventChatMsg,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Command { message } => {}
            _ => {}
        };
    }
}
