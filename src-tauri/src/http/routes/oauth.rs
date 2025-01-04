use std::sync::Arc;

use anyhow::Context;
use axum::{response::IntoResponse, Extension, Json};
use reqwest::header::CONTENT_TYPE;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use twitch_api::{
    helix::Scope,
    twitch_oauth2::{AccessToken, UserToken},
};

use crate::{
    database::entity::{twitch_access::SetTwitchAccess, TwitchAccessModel},
    http::error::HttpResult,
    twitch::manager::TwitchManager,
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
    access_token: AccessToken,
    scopes: Vec<Scope>,
}

/// POST /oauth/complete
///
/// Handles the completion of OAuth logging into the twitch account storing
/// the access token and authorized scopes
pub async fn handle_oauth_complete(
    Extension(db): Extension<DatabaseConnection>,
    Extension(twitch_manager): Extension<Arc<TwitchManager>>,
    Json(req): Json<OAuthComplete>,
) -> HttpResult<()> {
    let token =
        UserToken::from_existing(&twitch_manager.helix_client, req.access_token, None, None)
            .await
            .context("failed to create user token")?;

    let access_token = token.access_token.clone();
    let scopes = req.scopes;

    twitch_manager.set_authenticated(token).await;

    TwitchAccessModel::set(
        &db,
        SetTwitchAccess {
            access_token,
            scopes,
        },
    )
    .await?;

    Ok(Json(()))
}
