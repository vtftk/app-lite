use std::sync::Arc;

use http::server::EventRecvHandle;
use tauri::Manager;
use tokio::sync::broadcast;
use twitch::manager::TwitchManager;
use twitch_api::{
    eventsub::{event::websocket, WebsocketTransport},
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

    // Create the HelixClient, which is used to make requests to the Twitch API
    let client: HelixClient<reqwest::Client> = HelixClient::default();

    let (tx, rx) = broadcast::channel(10);
    let event_recv = EventRecvHandle(rx);

    tauri::Builder::default()
        .setup({
            // Copy shared auth state for the server
            let client = client.clone();
            let event_recv = event_recv.clone();

            move |app| {
                let handle = app.handle().clone();

                let (twitch_manager, mut twitch_event_rx) =
                    TwitchManager::new(client.clone(), handle.clone());
                let twitch_manager = Arc::new(twitch_manager);

                _ = tauri::async_runtime::spawn(async move {
                    while let Ok(event) = twitch_event_rx.recv().await {}
                });

                _ = tauri::async_runtime::spawn(async move {
                    _ = http::server::start(client, event_recv, handle, twitch_manager).await;
                });

                // TODO: Start server and block until a channel reports back that the server started?
                // store server task in a state variable to allow attempting restart within app
                Ok(())
            }
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::auth::get_twitch_oauth_uri,
            commands::auth::is_authenticated,
            commands::auth::open_twitch_oauth_uri,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
