use crate::{
    database::entity::{
        app_data::{AppData, AppDataModel},
        vt_access::SetVTAccess,
        VTAccessModel,
    },
    http::{
        error::{DynHttpError, HttpResult},
        models::{GetAuthTokenResponse, SetAuthTokenRequest},
    },
    state::runtime_app_data::{RuntimeAppData, RuntimeAppDataStore, UpdateRuntimeAppData},
};
use anyhow::Context;
use axum::{
    body::Body,
    extract::Path,
    http::{Response, StatusCode},
    Extension, Json,
};
use reqwest::header::{CACHE_CONTROL, CONTENT_TYPE};
use sea_orm::{DatabaseConnection, ModelTrait};
use tauri::{path::BaseDirectory, AppHandle, Manager};

/// GET /app-data
///
/// Obtain the current app data configuration. Contains stored
/// state such as calibration and throwables configuration
pub async fn get_app_data(Extension(db): Extension<DatabaseConnection>) -> HttpResult<AppData> {
    let data = AppDataModel::get_or_default(&db).await?;

    Ok(Json(data))
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

            if let Some(vtube_studio_auth) = req.vtube_studio_auth {
                runtime_app_data.vtube_studio_auth = vtube_studio_auth;
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
/// Retrieve the contents of a file from one of the content folders
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
        .header(CACHE_CONTROL, "public, max-age=31536000, immutable")
        .body(file_bytes.into())
        .context("failed to make response")?)
}

/// GET /defaults/:folder/:name
pub async fn get_defaults_file(
    Path((folder, name)): Path<(String, String)>,
    Extension(app): Extension<AppHandle>,
) -> Result<Response<Body>, DynHttpError> {
    let file_path = app
        .path()
        .resolve(format!("defaults/{folder}/{name}"), BaseDirectory::Resource)
        .context("failed to get file path")?;

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
        .header(CACHE_CONTROL, "public, max-age=31536000, immutable")
        .body(file_bytes.into())
        .context("failed to make response")?)
}

/// POST /data/set-auth-token
///
/// Set the current VTube Studio access token for the overlay
pub async fn handle_set_auth_token(
    Extension(db): Extension<DatabaseConnection>,
    Json(req): Json<SetAuthTokenRequest>,
) -> HttpResult<()> {
    if let Some(access_token) = req.auth_token {
        // Set new access token
        VTAccessModel::set(&db, SetVTAccess { access_token })
            .await
            .context("failed to update access")?;
    } else {
        // Clear existing access token
        let access = VTAccessModel::get(&db)
            .await
            .context("failed to get access")?;
        if let Some(access) = access {
            access
                .delete(&db)
                .await
                .context("failed to delete original token")?;
        }
    }

    Ok(Json(()))
}

/// GET /data/get-auth-token
///
/// Retrieve the current VTube Studio access token for the overlay
pub async fn handle_get_auth_token(
    Extension(db): Extension<DatabaseConnection>,
) -> HttpResult<GetAuthTokenResponse> {
    let access = VTAccessModel::get(&db)
        .await
        .context("failed to get access")?;

    Ok(Json(GetAuthTokenResponse {
        auth_token: access.map(|access| access.access_token),
    }))
}
