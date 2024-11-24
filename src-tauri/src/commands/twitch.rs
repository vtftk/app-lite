use anyhow::Context;
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
) -> CmdResult<Vec<CustomReward>> {
    let user_token = twitch_manager
        .get_user_token()
        .await
        .context("not authenticated")?;

    let broadcaster_id = &user_token.user_id;

    let rewards = twitch_manager
        .helix_client
        .get_all_custom_rewards(broadcaster_id, false, &user_token)
        .await
        .context("failed to load redeems")?;

    Ok(rewards)
}
