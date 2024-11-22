use std::{str::FromStr, sync::Arc};

use anyhow::Context;
use http::server::EventRecvHandle;
use log::debug;
use state::app_data::{load_app_data, AppDataStore};
use tauri::Manager;
use tokio::sync::broadcast;
use twitch::manager::TwitchManager;
use twitch_api::{
    twitch_oauth2::{AccessToken, UserToken},
    HelixClient,
};

mod commands;
mod constants;
mod http;
mod state;
mod twitch;

/// Prevent slow changes from macro by using a separate entrypoint
/// from the macro
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    run_inner();
}

fn run_inner() {
    env_logger::init();

    tauri::Builder::default()
        .setup(move |app| {
            // Create the HelixClient, which is used to make requests to the Twitch API
            let client: HelixClient<reqwest::Client> = HelixClient::default();

            let (event_tx, rx) = broadcast::channel(10);
            let event_recv = EventRecvHandle(rx);

            let app_data_path = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            let app_data_file = app_data_path.join("data.json");

            debug!("app data path: {:?}", app_data_path);

            let app_data = tauri::async_runtime::block_on(AppDataStore::load(app_data_file))
                .expect("failed to load app data");

            let handle = app.handle().clone();

            let (twitch_manager, mut twitch_event_rx) =
                TwitchManager::new(client.clone(), handle.clone());
            let twitch_manager = Arc::new(twitch_manager);

            app.manage(app_data.clone());
            app.manage(event_tx.clone());
            app.manage(twitch_manager.clone());

            _ = tauri::async_runtime::spawn({
                let twitch_manager = twitch_manager.clone();
                let helix_client = client.clone();
                let app_data = app_data.clone();

                async move {
                    {
                        let app_data = app_data.read().await;
                        if let Some(access_token) = app_data
                            .twitch
                            .access_token
                            .as_ref()
                            .and_then(|access_token| AccessToken::from_str(access_token).ok())
                        {
                            if let Ok(token) =
                                UserToken::from_existing(&helix_client, access_token, None, None)
                                    .await
                                    .context("failed to create user token")
                            {
                                twitch_manager.set_authenticated(token).await;
                            }
                        }
                    }

                    while let Ok(_event) = twitch_event_rx.recv().await {}
                }
            });

            _ = tauri::async_runtime::spawn(async move {
                _ = http::server::start(client, event_recv, handle, twitch_manager, app_data).await;
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
