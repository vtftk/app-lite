use anyhow::Context;
use constants::TWITCH_REQUIRED_SCOPES;
use database::{
    clean_old_data,
    entity::{app_data::AppDataModel, TwitchAccessModel},
};
use events::{
    create_event_channel, processing::process_twitch_events, scheduler::create_scheduler,
};
use log::{debug, error, info};
use script::{events::ScriptEventActor, runtime::create_script_executor};
use sea_orm::{DatabaseConnection, ModelTrait};
use state::runtime_app_data::RuntimeAppDataStore;
use std::sync::Arc;
use tauri::{async_runtime::block_on, Manager};
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
        // Shell access plugin
        .plugin(tauri_plugin_shell::init())
        // Don't allow creation of multiple windows, instead focus the existing window
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .setup(move |app| {
            let handle = app.handle().clone();

            let app_data_path = app
                .path()
                .app_data_dir()
                .context("failed to get app data dir")?;

            let db_file = app_data_path.join("app.db");

            let db =
                block_on(database::connect_database(&db_file)).expect("failed to load database");

            let (twitch_manager, twitch_event_rx) = TwitchManager::new(handle.clone());
            let (event_tx, event_rx) = create_event_channel();

            let runtime_app_data = RuntimeAppDataStore::new(handle.clone());

            let script_handle = create_script_executor();

            // Add auto updater plugin if auto updating is allowed
            {
                let auto_updating = block_on(AppDataModel::is_auto_updating(&db))?;
                if auto_updating {
                    handle.plugin(tauri_plugin_updater::Builder::new().build())?;
                }
            }

            // Run background cleanup
            tauri::async_runtime::spawn(clean_old_data(db.clone()));

            // Create background event scheduler
            let scheduler_handle = create_scheduler(
                db.clone(),
                twitch_manager.clone(),
                script_handle.clone(),
                event_tx.clone(),
            );

            // Provide runtime app data stores
            app.manage(runtime_app_data.clone());

            // Provide access to the scheduler
            app.manage(scheduler_handle);

            // Provide access to twitch manager and event sender
            app.manage(event_tx.clone());
            app.manage(twitch_manager.clone());

            // Provide access to script running and
            app.manage(script_handle.clone());

            // Provide database access
            app.manage(db.clone());

            // Attempt to authenticate with twitch using the saved token
            _ = tauri::async_runtime::spawn(attempt_twitch_auth_existing_token(
                db.clone(),
                twitch_manager.clone(),
            ));

            // Initialize script actor
            let actor = ScriptEventActor::new(event_tx.clone(), db.clone(), twitch_manager.clone());

            block_on(script::events::init_global_script_event_actor(actor));

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
                runtime_app_data,
            ));

            tray::create_tray_menu(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Calibration commands
            commands::calibration::set_calibration_step,
            commands::calibration::calibration_move_model,
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
            // Sound commands
            commands::sounds::get_sounds,
            commands::sounds::get_sound_by_id,
            commands::sounds::create_sound,
            commands::sounds::update_sound,
            commands::sounds::delete_sound,
            commands::sounds::update_sound_orderings,
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
            commands::events::get_event_logs,
            commands::events::delete_event_logs,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        // Prevent default exit handling, app exiting is done
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { api, code, .. } = event {
                let db = app.state::<DatabaseConnection>();
                let main_config = block_on(AppDataModel::get_main_config(db.inner()));
                let minimize_to_tray = main_config.is_ok_and(|value| value.minimize_to_tray);

                if code.is_none() && minimize_to_tray {
                    api.prevent_exit();
                }
            }
        });
}

/// Attempts to authenticate with twitch using an existing access token
async fn attempt_twitch_auth_existing_token(
    db: DatabaseConnection,
    twitch_manager: Arc<TwitchManager>,
) {
    let access = match TwitchAccessModel::get(&db).await {
        Ok(Some(value)) => value,
        Ok(None) => {
            debug!("not authenticated, skipping login");
            return;
        }
        Err(err) => {
            error!("failed to load twitch access: {err:?}");
            return;
        }
    };

    let access_token = access.access_token.0.clone();
    let scopes = &access.scopes.0;

    for required_scope in TWITCH_REQUIRED_SCOPES {
        if !scopes.contains(required_scope) {
            info!("logging out current access token, missing required scope");

            // Clear outdated / invalid access token
            _ = access.delete(&db).await;
            return;
        }
    }

    if let Err(err) = twitch_manager
        .attempt_auth_existing_token(access_token)
        .await
    {
        error!("stored access token is invalid: {}", err);

        // Clear outdated / invalid access token
        _ = access.delete(&db).await;
    }
}
