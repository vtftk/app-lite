use anyhow::Context;
use database::clean_old_data;
use events::{create_event_channel, processing::process_twitch_events};
use log::error;
use script::{events::ScriptEventActor, runtime::create_script_executor};
use state::{app_data::AppDataStore, runtime_app_data::RuntimeAppDataStore};
use std::sync::Arc;
use tauri::Manager;
use twitch::manager::TwitchManager;

mod commands;
mod constants;
mod database;
mod events;
mod http;
mod integrations;
mod script;
mod state;
mod tray;
mod twitch;

/// Prevent slow changes from macro by using a separate entrypoint
/// from the macro
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let handle = app.handle().clone();

            let app_data_path = app
                .path()
                .app_data_dir()
                .context("failed to get app data dir")?;
            let app_data_file = app_data_path.join("data.json");

            let db_file = app_data_path.join("app.db");

            let db = tauri::async_runtime::block_on(database::connect_database(&db_file))
                .expect("failed to load database");

            let (twitch_manager, twitch_event_rx) = TwitchManager::new(handle.clone());
            let (event_tx, event_rx) = create_event_channel();

            let app_data = tauri::async_runtime::block_on(AppDataStore::load(app_data_file))
                .expect("failed to load app data");

            let runtime_app_data = RuntimeAppDataStore::new(handle.clone());

            let script_handle = create_script_executor();

            // Run background cleanup
            tauri::async_runtime::spawn(clean_old_data(db.clone(), app_data.clone()));

            // Provide app data and runtime app data stores
            app.manage(app_data.clone());
            app.manage(runtime_app_data.clone());

            // Provide access to twitch manager and event sender
            app.manage(event_tx.clone());
            app.manage(twitch_manager.clone());

            // Provide access to script running and
            app.manage(script_handle.clone());

            // Provide database access
            app.manage(db.clone());

            // Attempt to authenticate with twitch using the saved token
            _ = tauri::async_runtime::spawn(attempt_twitch_auth_existing_token(
                app_data.clone(),
                twitch_manager.clone(),
            ));

            // Initialize script actor
            let actor = ScriptEventActor::new(
                app_data.clone(),
                event_tx.clone(),
                db.clone(),
                twitch_manager.clone(),
            );

            tauri::async_runtime::block_on(script::events::init_global_script_event_actor(actor));

            // Handle events triggered by twitch
            _ = tauri::async_runtime::spawn(process_twitch_events(
                db.clone(),
                twitch_manager.clone(),
                script_handle,
                event_tx.clone(),
                twitch_event_rx,
            ));

            // Run HTTP server
            _ = tauri::async_runtime::spawn(http::server::start(
                db,
                event_rx,
                handle,
                twitch_manager,
                app_data,
                runtime_app_data,
            ));

            tray::create_tray_menu(app)?;

            // TODO: Start server and block until a channel reports back that the server started?
            // store server task in a state variable to allow attempting restart within app
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Calibration commands
            commands::calibration::set_calibration_step,
            commands::calibration::get_calibration_data,
            // Testing and running commands
            commands::test::test_throw,
            commands::test::test_throw_barrage,
            commands::test::test_sound,
            commands::test::detect_vtube_studio,
            // Data manipulation comments
            commands::data::get_app_data,
            commands::data::get_runtime_app_data,
            commands::data::set_app_data,
            commands::data::upload_file,
            commands::data::update_hotkeys,
            commands::data::get_overlay_url,
            // Twitch commands
            commands::twitch::get_twitch_oauth_uri,
            commands::twitch::is_authenticated,
            commands::twitch::open_twitch_oauth_uri,
            commands::twitch::logout,
            commands::twitch::get_redeems_list,
            commands::twitch::refresh_redeems_list,
            // Item manipulation commands
            commands::items::get_item_by_id,
            commands::items::get_items,
            commands::items::get_item_sounds,
            commands::items::create_item,
            commands::items::update_item,
            commands::items::update_item_orderings,
            commands::items::delete_item,
            commands::items::append_item_impact_sounds,
            // Item collection commands
            commands::items::get_item_collections,
            commands::items::get_item_collection,
            commands::items::update_item_collection_orderings,
            commands::items::update_item_collection_item_orderings,
            commands::items::set_item_collection_items,
            commands::items::append_item_collection_items,
            commands::items::delete_item_collection,
            // Sound commands
            commands::sounds::get_sounds,
            commands::sounds::get_sound_by_id,
            commands::sounds::create_sound,
            commands::sounds::update_sound,
            commands::sounds::delete_sound,
            commands::sounds::update_sound_orderings,
            // Script commands
            commands::scripts::get_scripts,
            commands::scripts::get_script_by_id,
            commands::scripts::create_script,
            commands::scripts::update_script,
            commands::scripts::delete_script,
            commands::scripts::get_script_logs,
            commands::scripts::delete_script_logs,
            commands::scripts::update_script_orderings,
            commands::scripts::get_script_events,
            // Command commands
            commands::commands::get_commands,
            commands::commands::get_command_by_id,
            commands::commands::create_command,
            commands::commands::update_command,
            commands::commands::delete_command,
            commands::commands::get_command_logs,
            commands::commands::delete_command_logs,
            commands::commands::update_command_orderings,
            commands::commands::get_command_executions,
            commands::commands::delete_command_executions,
            // Event commands
            commands::events::get_events,
            commands::events::get_event_by_id,
            commands::events::create_event,
            commands::events::update_event,
            commands::events::delete_event,
            commands::events::test_event_by_id,
            commands::events::update_event_orderings,
            commands::events::get_event_executions,
            commands::events::delete_event_executions,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        // Prevent default exit handling, app exiting is done
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { api, code, .. } = event {
                let app_data = app.state::<AppDataStore>();
                let app_data = &*tauri::async_runtime::block_on(app_data.read());

                if code.is_none() && app_data.main_config.minimize_to_tray {
                    api.prevent_exit();
                }
            }
        });
}

/// Attempts to authenticate with twitch using an existing access token
async fn attempt_twitch_auth_existing_token(
    app_data_store: AppDataStore,
    twitch_manager: Arc<TwitchManager>,
) {
    // Read existing access token
    let access_token = {
        let app_data = app_data_store.read().await;
        match app_data.twitch_config.access_token.clone() {
            Some(value) => value,
            None => return,
        }
    };

    if let Err(err) = twitch_manager
        .attempt_auth_existing_token(access_token)
        .await
    {
        error!("stored access token is invalid: {}", err);

        // Clear outdated / invalid access token
        _ = app_data_store
            .write(|app_data| {
                app_data.twitch_config.access_token = None;
            })
            .await;
    }
}
