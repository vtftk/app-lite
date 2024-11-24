use crate::{
    http::error::DynHttpError,
    state::{
        app_data::{AppData, AppDataStore},
        runtime_app_data::{RuntimeAppData, RuntimeAppDataStore, UpdateRuntimeAppData},
    },
};
use anyhow::Context;
use axum::{
    body::Body,
    extract::Path,
    http::{Response, StatusCode},
    Extension, Json,
};
use reqwest::header::CONTENT_TYPE;
use tauri::{AppHandle, Manager};

/// GET /app-data
///
/// Obtain the current app data configuration. Contains stored
/// state such as calibration and throwables configuration
pub async fn get_app_data(Extension(app_data): Extension<AppDataStore>) -> Json<AppData> {
    let mut data = app_data.read().await.clone();

    // Hide twitch access token from frontend
    data.twitch_config.access_token = None;

    Json(data)
}

/// GET /runtime-app-data
///
/// Get the current state of the app at runtime, contains details about
/// the current state, i.e current model ID
///
/// Data stored in runtime app data store is reset when the server restarts
pub async fn get_runtime_data(
    Extension(runtime_app_data): Extension<RuntimeAppDataStore>,
) -> Json<RuntimeAppData> {
    Json(runtime_app_data.read().await.clone())
}

/// PUT /runtime-app-data
///
/// Sets the current state of the app at runtime
pub async fn update_runtime_data(
    Extension(runtime_app_data): Extension<RuntimeAppDataStore>,
    Json(req): Json<UpdateRuntimeAppData>,
) -> StatusCode {
    // Update the stored runtime data
    runtime_app_data
        .write(|runtime_app_data| {
            if let Some(model_id) = req.model_id {
                runtime_app_data.model_id = model_id;
            }

            if let Some(vtube_studio_connected) = req.vtube_studio_connected {
                runtime_app_data.vtube_studio_connected = vtube_studio_connected;
            }

            if let Some(hotkeys) = req.hotkeys {
                runtime_app_data.hotkeys = hotkeys;
            }
        })
        .await;

    StatusCode::OK
}

/// GET /content/:folder/:name
///
/// Sets the current state of the app at runtime
pub async fn get_content_file(
    Path((folder, name)): Path<(String, String)>,
    Extension(app): Extension<AppHandle>,
) -> Result<Response<Body>, DynHttpError> {
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;
    let content_path = app_data_path.join("content");
    let file_path = content_path.join(folder).join(name);

    if !file_path.exists() {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(vec![].into())
            .context("failed to make response")?);
    }

    let mime = mime_guess::from_path(&file_path);

    let file_bytes = tokio::fs::read(file_path)
        .await
        .context("failed to read content file")?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime.first_or_octet_stream().essence_str())
        .body(file_bytes.into())
        .context("failed to make response")?)
}
