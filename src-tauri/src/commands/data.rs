use crate::{
    constants::LOCAL_SERVER_PORT,
    events::EventMessage,
    state::{
        app_data::{AppData, AppDataStore},
        runtime_app_data::{RuntimeAppData, RuntimeAppDataStore},
    },
};
use serde::Deserialize;
use std::path::Path;
use tauri::{AppHandle, Manager};
use tokio::sync::broadcast;
use uuid::Uuid;

#[tauri::command]
pub fn update_hotkeys(
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> Result<bool, ()> {
    event_sender
        .send(EventMessage::UpdateHotkeys)
        .map_err(|_| ())?;

    Ok(true)
}

#[tauri::command]
pub fn get_overlay_url() -> String {
    format!("http://localhost:{}/oauth", LOCAL_SERVER_PORT)
}

#[tauri::command]
pub async fn get_app_data(app_data: tauri::State<'_, AppDataStore>) -> Result<AppData, ()> {
    Ok(app_data.read().await.clone())
}

#[tauri::command]
pub async fn get_runtime_app_data(
    runtime_app_data: tauri::State<'_, RuntimeAppDataStore>,
) -> Result<RuntimeAppData, ()> {
    Ok(runtime_app_data.read().await.clone())
}

#[tauri::command]
pub async fn set_app_data(
    app_data: AppData,
    app_data_store: tauri::State<'_, AppDataStore>,
) -> Result<bool, ()> {
    _ = app_data_store
        .write(|old_app_data| *old_app_data = app_data)
        .await;

    Ok(true)
}

#[derive(Debug, Deserialize)]
pub enum FileType {
    ThrowableImage,
    ImpactSound,
    ImpactImage,
    Sound,
}

fn get_type_folder(file_type: FileType) -> &'static str {
    match file_type {
        FileType::ThrowableImage => "throwable_images",
        FileType::ImpactSound => "impact_sounds",
        FileType::ImpactImage => "impact_images",
        FileType::Sound => "sounds",
    }
}

#[tauri::command]
pub async fn upload_file(
    file_type: FileType,
    file_name: String,
    file_data: Vec<u8>,
    app: AppHandle,
) -> Result<String, ()> {
    let app_data_path = app
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");
    let content_path = app_data_path.join("content");

    let type_folder = get_type_folder(file_type);
    let type_folder_path = content_path.join(type_folder);

    if !type_folder_path.exists() {
        tokio::fs::create_dir_all(&type_folder_path)
            .await
            .expect("failed to create content folder");
    }

    let file_path_name = Path::new(&file_name);
    let extension = file_path_name
        .extension()
        .expect("missing file extension")
        .to_string_lossy();

    let file_id = Uuid::new_v4();
    let file_name = format!("{}.{}", file_id, extension);

    let file_path = type_folder_path.join(&file_name);

    tokio::fs::write(&file_path, file_data)
        .await
        .expect("save file");

    let url = format!("backend://content/{}/{}", type_folder, file_name);

    Ok(url)
}
