use crate::script::events::{global_script_event, TwitchCredentials, TwitchGetCredentials};
use anyhow::Context;
use deno_core::*;

/// Get twitch credentials within JS
#[op2(async)]
#[serde]
pub async fn op_twitch_get_credentials() -> anyhow::Result<Option<TwitchCredentials>> {
    global_script_event(TwitchGetCredentials)
        .await
        .context("failed to send event")?
}
