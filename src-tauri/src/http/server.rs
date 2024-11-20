//! # Server
//!
//! Internal server for handling OAuth responses and serving the app overlay HTML

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use anyhow::Context;
use axum::{
    extract::WebSocketUpgrade,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use twitch_api::{
    twitch_oauth2::{AccessToken, UserToken},
    HelixClient,
};

use crate::{constants::LOCAL_SERVER_PORT, state::auth::SharedAuthState};

use super::{
    error::HttpResult,
    ws::{
        handle_socket, AuthStateChange, EventRecvHandle, EventSendHandle, WebsocketServerMessage,
    },
};

pub async fn start(
    auth_state: SharedAuthState,
    helix_client: HelixClient<'static, reqwest::Client>,
    event_handles: (EventSendHandle, EventRecvHandle),
) {
    // build our application with a single route
    let app = Router::new()
        .route("/oauth", get(handle_oauth))
        .route("/oauth/complete", post(handle_oauth_complete))
        .route("/ws", get(handle_ws))
        .layer(Extension(auth_state))
        .layer(Extension(helix_client))
        .layer(Extension(event_handles.0))
        .layer(Extension(event_handles.1));

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, LOCAL_SERVER_PORT));

    // run our app with hyper, listening globally on port 3000
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
    Extension(event_handle): Extension<EventSendHandle>,
    Extension(auth_state): Extension<SharedAuthState>,
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

    auth_state.set_authenticated(token).await;

    event_handle.send(WebsocketServerMessage::AuthStateChange {
        state: AuthStateChange::Authenticated,
    });

    Ok(Json(()))
}

async fn handle_ws(
    Extension(event_handle): Extension<EventRecvHandle>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, event_handle))
}
