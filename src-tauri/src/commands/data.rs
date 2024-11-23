use tokio::sync::broadcast;

use crate::{
    events::EventMessage,
    state::{
        app_data::{AppData, AppDataStore, ThrowableConfig},
        runtime_app_data::{RuntimeAppData, RuntimeAppDataStore},
    },
};

#[tauri::command]
pub async fn get_app_data(app_data: tauri::State<'_, AppDataStore>) -> Result<AppData, ()> {
    let mut data = app_data.read().await.clone();

    // Hide twitch access token from frontend
    data.twitch.access_token = None;

    Ok(data)
}

#[tauri::command]
pub async fn get_runtime_app_data(
    runtime_app_data: tauri::State<'_, RuntimeAppDataStore>,
) -> Result<RuntimeAppData, ()> {
    Ok(runtime_app_data.read().await.clone())
}
