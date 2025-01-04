use crate::{
    commands::CmdResult,
    database::entity::app_data::AppDataModel,
    events::EventMessage,
    state::{
        app_data::AppData,
        runtime_app_data::{RuntimeAppData, RuntimeAppDataStore},
    },
};
use anyhow::Context;
use log::{debug, error};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use std::{path::Path, str::FromStr};
use tauri::{AppHandle, Manager, Url};
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
pub async fn get_overlay_url(db: tauri::State<'_, DatabaseConnection>) -> CmdResult<String> {
    let http_port = AppDataModel::get_http_port(db.inner()).await?;
    Ok(format!("http://localhost:{}/overlay", http_port))
}

#[tauri::command]
pub async fn get_app_data(db: tauri::State<'_, DatabaseConnection>) -> CmdResult<AppData> {
    Ok(AppDataModel::get_or_default(db.inner()).await?)
}

#[tauri::command]
pub async fn get_runtime_app_data(
    runtime_app_data: tauri::State<'_, RuntimeAppDataStore>,
) -> CmdResult<RuntimeAppData> {
    Ok(runtime_app_data.read().await.clone())
}

#[tauri::command]
pub async fn set_app_data(
    app_data: AppData,
    db: tauri::State<'_, DatabaseConnection>,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> CmdResult<bool> {
    let model = AppDataModel::set(db.inner(), app_data).await?;

    _ = event_sender.send(EventMessage::AppDataUpdated {
        app_data: Box::new(model.data.0),
    });

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
) -> CmdResult<String> {
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

    let file_path = type_folder_path.join(&file_name);

    tokio::fs::write(&file_path, file_data)
        .await
        .context("save file")?;

    let url = format!("backend://content/{}/{}", type_folder, file_name);

    Ok(url)
}

pub async fn delete_src_file(url: String, app_handle: AppHandle) -> anyhow::Result<()> {
    let url = match Url::from_str(&url) {
        Ok(value) => value,
        Err(err) => {
            error!("invalid src url: {err:?}");
            return Ok(());
        }
    };

    // Ignore non-backend URLs
    if url.scheme() != "backend" {
        return Ok(());
    }

    if url.domain().is_none_or(|value| !value.eq("content")) {
        return Ok(());
    }

    let file_path = url.path();

    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;

    let file_path = app_data_path
        .join("content")
        .join(file_path.strip_prefix("/").unwrap_or(file_path));

    debug!("attempt delete content: {:?} {:?}", url, file_path);

    if file_path.exists() {
        tokio::fs::remove_file(file_path)
            .await
            .context("failed to delete file")?;
    }

    Ok(())
}
