use std::path::Path;

use anyhow::Context;
use serde::Deserialize;
use tauri::{ipc::Request, AppHandle, Manager};
use tokio::sync::broadcast;
use uuid::Uuid;

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

#[derive(Debug, Deserialize)]
pub enum FileType {
    ThrowableImage,
    ImpactSound,
    ImpactImage,
}

fn get_type_folder(file_type: FileType) -> &'static str {
    match file_type {
        FileType::ThrowableImage => "throwable_images",
        FileType::ImpactSound => "impact_sounds",
        FileType::ImpactImage => "impact_images",
    }
}

#[tauri::command]
pub async fn upload_image_file(
    file_type: FileType,
    file_name: String,
    file_data: Vec<u8>,
    app: AppHandle,
) -> anyhow::Result<String> {
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;
    let content_path = app_data_path.join("content");

    let type_folder = get_type_folder(file_type);
    let type_folder_path = content_path.join(type_folder);

    if !type_folder_path.exists() {
        tokio::fs::create_dir_all(&type_folder_path)
            .await
            .context("failed to create content folder")?;
    }

    let file_path_name = Path::new(&file_name);
    let extension = file_path_name
        .extension()
        .context("missing file extension")?
        .to_string_lossy();

    let file_id = Uuid::new_v4();
    let file_name = format!("{}.{}", file_id, extension);

    let file_path = type_folder_path.join(file_name);

    tokio::fs::write(&file_path, file_data);

    // let url = format!("{}/content/{}", URL file_name);

    Ok("".to_string())
}
