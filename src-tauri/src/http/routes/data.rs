use crate::state::{
    app_data::{AppData, AppDataStore},
    runtime_app_data::{RuntimeAppData, RuntimeAppDataStore},
};
use axum::{http::StatusCode, Extension, Json};

/// GET /app-data
///
/// Obtain the current app data configuration. Contains stored
/// state such as calibration and throwables configuration
pub async fn get_app_data(Extension(app_data): Extension<AppDataStore>) -> Json<AppData> {
    let mut data = app_data.read().await.clone();

    // Hide twitch access token from frontend
    data.twitch.access_token = None;

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

/// POST /runtime-app-data
///
/// Sets the current state of the app at runtime
pub async fn set_runtime_data(
    Extension(runtime_app_data): Extension<RuntimeAppDataStore>,
    Json(req): Json<RuntimeAppData>,
) -> StatusCode {
    // Update the stored runtime data
    runtime_app_data
        .write(|runtime_app_data| {
            *runtime_app_data = req.clone();
        })
        .await;

    StatusCode::OK
}
