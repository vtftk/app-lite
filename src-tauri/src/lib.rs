use std::sync::Arc;

use anyhow::Context;
use events::{EventMessage, EventRecvHandle};
use log::{debug, error};
use rand::{rngs::StdRng, SeedableRng};
use state::{
    app_data::{AppData, AppDataStore, EventConfig, EventOutcome, EventTrigger},
    runtime_app_data::RuntimeAppDataStore,
};
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
            commands::throw::test_throw,
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
            TwitchEvent::CheerBits(event) => {
                handle_cheer_bits_event(&events, &event_sender, event).await
            }
            TwitchEvent::Follow(event) => handle_follow_event(&events, &event_sender, event).await,
            TwitchEvent::Sub(event) => handle_sub_event(&events, &event_sender, event).await,
            TwitchEvent::GiftSub(event) => {
                handle_gift_sub_event(&events, &event_sender, event).await
            }
            TwitchEvent::ResubMsg(event) => handle_resub_event(&events, &event_sender, event).await,
            TwitchEvent::ChatMsg(event) => {
                handle_chat_msg_event(&events, &event_sender, event).await
            }
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
        execute_event_config(app_data.clone(), event_config.clone(), event_sender.clone());
    }
}

fn execute_event_config(
    app_data: Arc<AppData>,
    event_config: EventConfig,
    event_sender: broadcast::Sender<EventMessage>,
) {
    // Skip disabled events
    if !event_config.enabled {
        return;
    }

    // TODO: WAIT FOR COOLDOWN TO COMPLETE
    // TODO: CHECK USER HAS REQUIRED ROLE

    debug!("executing event outcome: {:?}", event_config);

    match event_config.outcome {
        EventOutcome::Random => {
            use rand::seq::SliceRandom;
            let mut rand = StdRng::from_entropy();
            let throwable = app_data.items.choose(&mut rand);
            let throwable = match throwable {
                Some(value) => value,
                // Throwable no longer exists
                None => return,
            };

            _ = event_sender.send(EventMessage::Throw {
                config: throwable.clone(),
            });
        }
        EventOutcome::RandomBarrage => {
            use rand::seq::SliceRandom;
            let mut rand = StdRng::from_entropy();
            let throwables = app_data.items.choose_multiple(&mut rand, 10);

            // TODO: Optimize by sending config and amount instead of duplicate configs

            _ = event_sender.send(EventMessage::ThrowDifferent {
                configs: throwables.cloned().collect(),
            });
        }
        EventOutcome::Throwable { throwable_id } => {
            let throwable = app_data.items.iter().find(|item| item.id == throwable_id);
            let throwable = match throwable {
                Some(value) => value,
                // Throwable no longer exists
                None => return,
            };

            _ = event_sender.send(EventMessage::Throw {
                config: throwable.clone(),
            });
        }
        EventOutcome::ThrowableBarrage { throwable_id } => {
            let throwable = app_data.items.iter().find(|item| item.id == throwable_id);
            let throwable = match throwable {
                Some(value) => value,
                // Throwable no longer exists
                None => return,
            };

            _ = event_sender.send(EventMessage::ThrowMany {
                config: throwable.clone(),
                amount: 10,
            });
        }
        EventOutcome::Collection { collection_id } => {}
        EventOutcome::TriggerHotkey { hotkey_id } => {
            _ = event_sender.send(EventMessage::TriggerHotkey { hotkey_id });
        }
        EventOutcome::PlaySound { sound_id } => {
            let sound = app_data.sounds.iter().find(|item| item.id == sound_id);
            let sound = match sound {
                Some(value) => value,
                // Throwable no longer exists
                None => return,
            };
            _ = event_sender.send(EventMessage::PlaySound {
                config: sound.clone(),
            });
        }
    }
}

async fn handle_cheer_bits_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventCheerBits,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Bits {
                max_throws,
                min_bits,
            } => todo!(),
            _ => {}
        };
    }
}

async fn handle_follow_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventFollow,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Follow => todo!(),
            _ => {}
        };
    }
}

async fn handle_sub_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventSub,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Subscription => todo!(),
            _ => {}
        };
    }
}

async fn handle_gift_sub_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventGiftSub,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::GiftedSubscription => todo!(),
            _ => {}
        };
    }
}

async fn handle_resub_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventReSub,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Subscription => todo!(),
            _ => {}
        };
    }
}

async fn handle_chat_msg_event(
    events: &[EventConfig],
    event_sender: &broadcast::Sender<EventMessage>,
    event: TwitchEventChatMsg,
) {
    for event in events {
        let trigger = match &event.trigger {
            state::app_data::EventTrigger::Command { message } => todo!(),
            _ => {}
        };
    }
}
