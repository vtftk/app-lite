use crate::commands::CmdResult;
use crate::constants::{TWITCH_CLIENT_ID, TWITCH_REQUIRED_SCOPES};
use crate::database::entity::app_data::AppDataModel;
use crate::twitch::manager::Twitch;
use anyhow::Context;
use log::debug;
use reqwest::Url;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tauri::State;
use twitch_api::helix::points::CustomReward;
use twitch_api::twitch_oauth2::{ClientId, ImplicitUserTokenBuilder};

/// Requests the list of available redeems from the broadcasters channel.
///
/// Used on the frontend for the dropdown menu that allows you to pick
/// from the list of redeems as an event trigger
#[tauri::command]
pub async fn get_redeems_list(twitch: State<'_, Twitch>) -> CmdResult<Arc<[CustomReward]>> {
    Ok(twitch
        .get_rewards_list()
        .await
        .context("failed to load redeems")?)
}

/// Reloads the list of available redeems
#[tauri::command]
pub async fn refresh_redeems_list(twitch: State<'_, Twitch>) -> CmdResult<bool> {
    debug!("reloading rewards list");
    _ = twitch.load_rewards_list().await;
    Ok(true)
}

/// Obtain a URL for use logging into twitch using OAuth2
#[tauri::command]
pub async fn get_twitch_oauth_uri(db: tauri::State<'_, DatabaseConnection>) -> CmdResult<String> {
    let http_port = AppDataModel::get_http_port(db.inner()).await?;

    let redirect_url = format!("http://localhost:{http_port}/oauth",);
    let redirect_url = Url::parse(&redirect_url).context("invalid redirect_uri")?;

    let (url, _csrf) =
        ImplicitUserTokenBuilder::new(ClientId::from_static(TWITCH_CLIENT_ID), redirect_url)
            .set_scopes(TWITCH_REQUIRED_SCOPES.to_vec())
            .generate_url();

    Ok(url.to_string())
}

#[tauri::command]
pub async fn is_authenticated(twitch: tauri::State<'_, Twitch>) -> Result<bool, ()> {
    Ok(twitch.is_authenticated().await)
}

#[tauri::command]
pub async fn logout(twitch: tauri::State<'_, Twitch>) -> Result<(), ()> {
    twitch.reset().await;
    Ok(())
}
