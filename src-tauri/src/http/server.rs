//! # Server
//!
//! Internal server for handling OAuth responses and serving the app overlay HTML

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use super::error::HttpResult;
use crate::constants::LOCAL_SERVER_PORT;
use crate::twitch::manager::TwitchManager;
use anyhow::Context;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use futures_util::stream::Stream;
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
) {
    // build our application with a single route
    let app = Router::new()
        .route("/oauth", get(handle_oauth))
        .route("/oauth/complete", post(handle_oauth_complete))
        .route("/events", get(handle_sse))
        .route("/calibration", post(handle_calibration_progress))
        .layer(Extension(helix_client))
        .layer(Extension(event_handles))
        .layer(Extension(app_handle))
        .layer(Extension(twitch_manager))
        .layer(CorsLayer::very_permissive());

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, LOCAL_SERVER_PORT));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

    twitch_manager.set_authenticated(token).await;

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

pub async fn handle_calibration_progress(
    Extension(app_handle): Extension<AppHandle>,
    Json(req): Json<CalibrationStepData>,
) -> HttpResult<()> {
    match &req {
        CalibrationStepData::NotStarted => {}
        CalibrationStepData::Smallest => todo!(),
        CalibrationStepData::Largest => todo!(),
        CalibrationStepData::Complete {
            smallest_point,
            largest_point,
        } => todo!(),
    }

    app_handle
        .emit("calibration_state", req)
        .context("failed to inform app")?;

    Ok(Json(()))
}

pub struct EventRecvHandle(pub broadcast::Receiver<EventMessage>);

impl Clone for EventRecvHandle {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

#[derive(Debug, Clone, Serialize)]
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
