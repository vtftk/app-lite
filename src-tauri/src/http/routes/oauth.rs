use std::sync::Arc;

use anyhow::Context;
use axum::{response::IntoResponse, Extension, Json};
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use twitch_api::twitch_oauth2::{AccessToken, UserToken};

use crate::{
    http::error::HttpResult, state::app_data::AppDataStore, twitch::manager::TwitchManager,
};

/// Embedded oauth response page for handling sending the fragment
static OAUTH_RESPONSE_PAGE: &str = include_str!("../resources/twitch-oauth-response.html");

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
    Json(req): Json<OAuthComplete>,
) -> HttpResult<()> {
    let token = UserToken::from_existing(
        &twitch_manager.helix_client,
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
            app_data.twitch_config.access_token = Some(access_token);
        })
        .await
        .context("saving app data")?;

    Ok(Json(()))
}
