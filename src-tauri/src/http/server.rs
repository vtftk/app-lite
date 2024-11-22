//! # Server
//!
//! Internal server for handling OAuth responses and serving the app overlay HTML

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use super::error::HttpResult;
use crate::constants::LOCAL_SERVER_PORT;
use crate::state::app_data::{AppData, AppDataStore, MinMax, ModelData, ModelId};
use crate::twitch::manager::TwitchManager;
use anyhow::Context;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use futures_util::stream::Stream;
use log::info;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::cors::CorsLayer;
use twitch_api::{
    twitch_oauth2::{AccessToken, UserToken},
    HelixClient,
};

pub async fn start(
    helix_client: HelixClient<'static, reqwest::Client>,
    event_handles: EventRecvHandle,
    app_handle: AppHandle,
    twitch_manager: Arc<TwitchManager>,
    app_data: AppDataStore,
) {
    // build our application with a single route
    let app = Router::new()
        .route("/oauth", get(handle_oauth))
        .route("/oauth/complete", post(handle_oauth_complete))
        .route("/events", get(handle_sse))
        .route("/calibration", post(handle_calibration_progress))
        .route("/app-data", get(handle_app_data))
        .layer(Extension(helix_client))
        .layer(Extension(event_handles))
        .layer(Extension(app_handle))
        .layer(Extension(twitch_manager))
        .layer(Extension(app_data))
        .layer(CorsLayer::very_permissive());

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, LOCAL_SERVER_PORT));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// GET /app-data
///
/// Obtain the current app data configuration
pub async fn handle_app_data(Extension(app_data): Extension<AppDataStore>) -> Json<AppData> {
    let mut data = app_data.read().await.clone();

    // Hide twitch access token from frontend
    data.twitch.access_token = None;

    Json(data)
}

/// Embedded oauth response page for handling sending the fragment
static OAUTH_RESPONSE_PAGE: &str = include_str!("./resources/twitch-oauth-response.html");

/// GET /oauth
///
/// Handles an OAuth response from twitch
///
/// Web server does not support handling fragments so we send back a small
/// HTML page which sends us the token to
pub async fn handle_oauth() -> impl IntoResponse {
    ([(CONTENT_TYPE, "text/html")], OAUTH_RESPONSE_PAGE)
}

#[derive(Debug, Deserialize)]
pub struct OAuthComplete {
    access_token: String,
}

/// POST /oauth/complete
///
pub async fn handle_oauth_complete(
    Extension(app_data): Extension<AppDataStore>,
    Extension(twitch_manager): Extension<Arc<TwitchManager>>,
    Extension(helix_client): Extension<HelixClient<'static, reqwest::Client>>,
    Json(req): Json<OAuthComplete>,
) -> HttpResult<()> {
    let token = UserToken::from_existing(
        &helix_client,
        AccessToken::from(req.access_token),
        None,
        None,
    )
    .await
    .context("failed to create user token")?;

    let access_token = token.access_token.as_str().to_string();
    twitch_manager.set_authenticated(token).await;

    app_data
        .write(|app_data| {
            app_data.twitch.access_token = Some(access_token);
        })
        .await
        .context("saving app data")?;

    Ok(Json(()))
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "step")]
pub struct CalibrationPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "step")]
pub enum CalibrationStepData {
    NotStarted,
    Smallest,
    Largest,
    Complete {
        model_id: ModelId,
        smallest_point: CalibrationPoint,
        largest_point: CalibrationPoint,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CalibrationStep {
    NotStarted,
    Smallest,
    Largest,
    Complete,
}

#[derive(Debug, Serialize)]
pub struct CalibrationProgressRes {}

pub async fn handle_calibration_progress(
    Extension(app_data): Extension<AppDataStore>,
    Extension(app_handle): Extension<AppHandle>,
    Json(req): Json<CalibrationStepData>,
) -> HttpResult<CalibrationProgressRes> {
    match &req {
        CalibrationStepData::NotStarted => {}
        CalibrationStepData::Smallest => {}
        CalibrationStepData::Largest => {}
        CalibrationStepData::Complete {
            model_id,
            smallest_point,
            largest_point,
        } => {
            info!(
                "COMPLETED CALIBRATION: {:?} {:?}",
                smallest_point, largest_point
            );

            app_data
                .write(move |app_data| {
                    app_data.models.insert(
                        model_id.to_string(),
                        ModelData {
                            x: MinMax {
                                min: smallest_point.x,
                                max: largest_point.x,
                            },
                            y: MinMax {
                                min: smallest_point.y,
                                max: largest_point.y,
                            },
                        },
                    );
                })
                .await
                .context("saving app data")?;
        }
    }

    app_handle
        .emit("calibration_state", req)
        .context("failed to inform app")?;

    Ok(Json(CalibrationProgressRes {}))
}

pub struct EventRecvHandle(pub broadcast::Receiver<EventMessage>);

impl Clone for EventRecvHandle {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum EventMessage {
    // Tells any connected browser apps to refresh
    Refresh,
    // Sets the current calibration step
    SetCalibrationStep { step: CalibrationStep },
}

async fn handle_sse(
    Extension(event_handle): Extension<EventRecvHandle>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    use tokio_stream::StreamExt;

    let stream = BroadcastStream::new(event_handle.0);
    let stream = stream.filter_map(|result| {
        result
            .ok()
            .and_then(|event| Event::default().json_data(event).ok())
            .map(Ok)
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
