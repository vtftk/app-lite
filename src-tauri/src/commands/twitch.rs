use anyhow::Context;
use log::debug;
use std::sync::Arc;
use tauri::State;
use twitch_api::helix::points::CustomReward;

use crate::{commands::CmdResult, twitch::manager::TwitchManager};

/// Requests the list of available redeems from the broadcasters channel.
///
/// Used on the frontend for the dropdown menu that allows you to pick
/// from the list of redeems as an event trigger
#[tauri::command]
pub async fn get_redeems_list(
    twitch_manager: State<'_, Arc<TwitchManager>>,
) -> CmdResult<Arc<[CustomReward]>> {
    Ok(twitch_manager
        .get_rewards_list()
        .await
        .context("failed to load redeems")?)
}

/// Reloads the list of available redeems
#[tauri::command]
pub async fn refresh_redeems_list(
    twitch_manager: State<'_, Arc<TwitchManager>>,
) -> CmdResult<bool> {
    debug!("reloading rewards list");
    _ = twitch_manager.load_rewards_list().await;
    Ok(true)
}
