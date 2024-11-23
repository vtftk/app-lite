use axum::{Extension, Json};

use crate::state::app_data::{AppData, AppDataStore};

/// GET /app-data
///
/// Obtain the current app data configuration
pub async fn handle_app_data(Extension(app_data): Extension<AppDataStore>) -> Json<AppData> {
    let mut data = app_data.read().await.clone();

    // Hide twitch access token from frontend
    data.twitch.access_token = None;

    Json(data)
}
