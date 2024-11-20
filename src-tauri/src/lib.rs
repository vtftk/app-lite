use http::ws::create_event_handles;
use state::auth::SharedAuthState;
use twitch_api::HelixClient;

mod commands;
mod constants;
mod http;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create the HelixClient, which is used to make requests to the Twitch API
    let client: HelixClient<reqwest::Client> = HelixClient::default();

    let auth_state = SharedAuthState::default();

    let (event_send, event_recv) = create_event_handles();

    tauri::Builder::default()
        .setup({
            // Copy shared auth state for the server
            let auth_state = auth_state.clone();
            let client = client.clone();
            let event_handles = (event_send.clone(), event_recv.clone());

            move |_app| {
                _ = tauri::async_runtime::spawn(async move {
                    _ = http::server::start(auth_state, client, event_handles).await;
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
