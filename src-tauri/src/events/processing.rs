use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Context};
use chrono::TimeDelta;
use futures::{future::BoxFuture, stream::FuturesUnordered};
use log::{debug, error};
use sea_orm::{sqlx::types::chrono::Utc, DatabaseConnection};
use tokio::{sync::broadcast, try_join};
use twitch_api::types::UserId;

use crate::{
    database::entity::{
        command_executions::{CommandExecutionModel, CreateCommandExecution},
        commands::CommandOutcome,
        event_executions::{CreateEventExecution, EventExecutionModel},
        scripts::ScriptWithEvent,
        shared::MinimumRequireRole,
        EventModel,
    },
    events::matching::match_raid_event,
    script::runtime::{
        CommandContext, CommandContextUser, RuntimeExecutionContext, ScriptExecutorHandle,
    },
    twitch::manager::{TwitchEvent, TwitchManager},
};

use super::{
    matching::{
        match_chat_event, match_cheer_bits_event, match_follow_event,
        match_gifted_subscription_event, match_re_subscription_event, match_redeem_event,
        match_subscription_event, CommandWithContext, EventData, EventInputData, EventMatchingData,
    },
    outcome::produce_outcome_message,
    EventMessage,
};

pub async fn process_twitch_events(
    db: DatabaseConnection,
    twitch_manager: Arc<TwitchManager>,
    script_handle: ScriptExecutorHandle,
    event_sender: broadcast::Sender<EventMessage>,

    mut twitch_event_rx: broadcast::Receiver<TwitchEvent>,
) {
    while let Ok(event) = twitch_event_rx.recv().await {
        debug!("twitch event received: {:?}", event);

        tokio::spawn({
            let db = db.clone();
            let twitch_manager = twitch_manager.clone();
            let script_handle = script_handle.clone();
            let event_sender = event_sender.clone();

            async move {
                let result =
                    process_twitch_event(db, twitch_manager, script_handle, event_sender, event)
                        .await;

                if let Err(err) = result {
                    debug!("failed to process twitch event: {err:?}",);
                }
            }
        });
    }
}

async fn process_twitch_event(
    db: DatabaseConnection,
    twitch_manager: Arc<TwitchManager>,
    script_handle: ScriptExecutorHandle,
    event_sender: broadcast::Sender<EventMessage>,
    event: TwitchEvent,
) -> anyhow::Result<()> {
    let match_data: EventMatchingData = match event {
        // Matchable events
        TwitchEvent::Redeem(event) => match_redeem_event(&db, event).await?,
        TwitchEvent::CheerBits(event) => match_cheer_bits_event(&db, event).await?,
        TwitchEvent::Follow(event) => match_follow_event(&db, event).await?,
        TwitchEvent::Sub(event) => match_subscription_event(&db, event).await?,
        TwitchEvent::GiftSub(event) => match_gifted_subscription_event(&db, event).await?,
        TwitchEvent::ResubMsg(event) => match_re_subscription_event(&db, event).await?,
        TwitchEvent::ChatMsg(event) => match_chat_event(&db, event).await?,
        TwitchEvent::Raid(event) => match_raid_event(&db, event).await?,

        // Internal events
        TwitchEvent::ModeratorsChanged => {
            debug!("reloading mods list");
            twitch_manager.load_moderator_list().await?;
            return Ok(());
        }
        TwitchEvent::VipsChanged => {
            debug!("reloading vips list");
            twitch_manager.load_vip_list().await?;
            return Ok(());
        }
        TwitchEvent::RewardsChanged => {
            debug!("reloading rewards list");
            twitch_manager.load_rewards_list().await?;
            return Ok(());
        }
        TwitchEvent::Reset => {
            debug!("resetting twitch manager");
            twitch_manager.reset().await;
            return Ok(());
        }
    };

    let command_futures =
        match_data
            .commands
            .into_iter()
            .map(|command| -> BoxFuture<'_, anyhow::Result<()>> {
                Box::pin(execute_command(
                    &db,
                    &script_handle,
                    &twitch_manager,
                    command,
                    match_data.event_data.clone(),
                ))
            });

    let script_futures =
        match_data
            .scripts
            .into_iter()
            .map(|script| -> BoxFuture<'_, anyhow::Result<()>> {
                Box::pin(execute_script(
                    &script_handle,
                    script,
                    match_data.event_data.clone(),
                ))
            });

    let event_futures =
        match_data
            .events
            .into_iter()
            .map(|event| -> BoxFuture<'_, anyhow::Result<()>> {
                Box::pin(execute_event(
                    &db,
                    &twitch_manager,
                    &event_sender,
                    event,
                    match_data.event_data.clone(),
                ))
            });

    let mut futures = command_futures
        .chain(script_futures)
        .chain(event_futures)
        .collect::<FuturesUnordered<BoxFuture<'_, anyhow::Result<()>>>>();

    use futures::StreamExt;

    while let Some(result) = futures.next().await {
        if let Err(err) = result {
            error!("error while executing event outcome: {err:?}");
        }
    }

    Ok(())
}

pub async fn execute_command(
    db: &DatabaseConnection,
    script_handle: &ScriptExecutorHandle,
    twitch_manager: &Arc<TwitchManager>,
    command: CommandWithContext,
    event_data: EventData,
) -> anyhow::Result<()> {
    let EventInputData::Chat { message, .. } = &event_data.input_data else {
        return Err(anyhow!("Non chat input data provided for chat execute"));
    };

    // Ensure required role is present
    if !has_required_role(
        twitch_manager,
        event_data.user.as_ref().map(|user| user.id.clone()),
        &command.command.require_role,
    )
    .await
    {
        debug!("skipping event: missing required role");
        return Ok(());
    }

    let current_time = Utc::now();

    let last_execution = command
        .command
        .last_execution(db)
        .await
        .context("failed to request last execution for command")?;

    // Ensure cooldown is not active
    if let Some(last_execution) = last_execution {
        let cooldown_end_time = last_execution
            .created_at
            .checked_add_signed(TimeDelta::milliseconds(command.command.cooldown as i64))
            .context("cooldown finishes too far in the future to compute")?;

        if current_time < cooldown_end_time {
            debug!("skipping command: cooldown");
            return Ok(());
        }
    }

    // Create metadata for storage
    let metadata =
        serde_json::to_value(&event_data).context("failed to serialize event metadata")?;

    let user = match event_data.user {
        Some(value) => value,
        None => return Err(anyhow!("failed to get twitch user")),
    };

    match command.command.outcome {
        CommandOutcome::Template { message } => {
            let to_usr = command
                .args
                .first()
                .map(|value| value.as_str())
                .unwrap_or_default();
            let message = message
                .replace("$(user)", user.name.as_str())
                .replace("$(touser)", to_usr);

            if message.len() < 500 {
                twitch_manager.send_chat_message(&message).await?;
            } else {
                let mut chars = message.chars();

                loop {
                    let message = chars.by_ref().take(500).collect::<String>();
                    if message.is_empty() {
                        break;
                    }

                    twitch_manager.send_chat_message(&message).await?;
                }
            }
        }
        CommandOutcome::Script { script } => {
            let user = CommandContextUser {
                id: user.id,
                name: user.name,
                display_name: user.display_name,
            };

            let ctx = CommandContext {
                full_message: message.to_string(),
                input_data: event_data.input_data,
                message: command.message,
                args: command.args,
                user,
            };

            script_handle
                .execute_command(
                    RuntimeExecutionContext::Command {
                        command_id: command.command.id,
                    },
                    script,
                    ctx,
                )
                .await?;
        }
    }

    // Store command execution
    CommandExecutionModel::create(
        db,
        CreateCommandExecution {
            command_id: command.command.id,
            created_at: current_time,
            metadata,
        },
    )
    .await
    .context("failed to store last command execution")?;

    Ok(())
}

pub async fn execute_script(
    script_handle: &ScriptExecutorHandle,
    script: ScriptWithEvent,
    event_data: EventData,
) -> anyhow::Result<()> {
    script_handle
        .execute(
            RuntimeExecutionContext::Script {
                script_id: script.script.id,
            },
            script.script.script,
            script.event,
            event_data,
        )
        .await?;
    Ok(())
}

pub async fn execute_event(
    db: &DatabaseConnection,
    twitch_manager: &Arc<TwitchManager>,
    event_sender: &broadcast::Sender<EventMessage>,
    event: EventModel,
    event_data: EventData,
) -> anyhow::Result<()> {
    // Ensure required role is present
    if !has_required_role(
        twitch_manager,
        event_data.user.as_ref().map(|value| value.id.clone()),
        &event.require_role,
    )
    .await
    {
        debug!("skipping event: missing required role");
        return Ok(());
    }

    let current_time = Utc::now();

    let last_execution = event
        .last_execution(db)
        .await
        .context("failed to request last execution for event")?;

    // Ensure cooldown is not active
    if let Some(last_execution) = last_execution {
        let cooldown_end_time = last_execution
            .created_at
            .checked_add_signed(TimeDelta::milliseconds(event.cooldown as i64))
            .context("cooldown finishes too far in the future to compute")?;

        if current_time < cooldown_end_time {
            debug!("skipping event: cooldown");
            return Ok(());
        }
    }

    // Create metadata for storage
    let metadata =
        serde_json::to_value(&event_data).context("failed to serialize event metadata")?;

    // Wait for outcome delay
    tokio::time::sleep(Duration::from_millis(event.outcome_delay as u64)).await;

    // Produce outcome message and send it
    if let Some(msg) =
        produce_outcome_message(db, twitch_manager, event_data, event.outcome).await?
    {
        _ = event_sender.send(msg);
    }

    // Store event execution
    EventExecutionModel::create(
        db,
        CreateEventExecution {
            event_id: event.id,
            created_at: current_time,
            metadata,
        },
    )
    .await
    .context("failed to store last event execution")?;

    Ok(())
}

pub async fn has_required_role(
    twitch_manager: &TwitchManager,
    user_id: Option<UserId>,
    required_role: &MinimumRequireRole,
) -> bool {
    // No role required (Initial level)
    if let MinimumRequireRole::None = required_role {
        return true;
    }

    // User must be authenticated at this point
    let user = match user_id {
        Some(user) => user,
        None => return false,
    };

    // User is the broadcaster
    if twitch_manager
        .get_user_token()
        .await
        .is_some_and(|value| value.user_id == user)
    {
        return true;
    }

    // Check VIP and moderator lists
    if let MinimumRequireRole::Vip = required_role {
        return try_join!(
            twitch_manager.get_vip_list(),
            twitch_manager.get_moderator_list()
        )
        .is_ok_and(|(vips, mods)| {
            vips.iter().any(|vip| vip.user_id == user)
                || mods.iter().any(|mods| mods.user_id == user)
        });
    }

    // Check just moderator list
    if let MinimumRequireRole::Mod = required_role {
        return twitch_manager
            .get_moderator_list()
            .await
            .is_ok_and(|mods| mods.iter().any(|mods| mods.user_id == user));
    }

    false
}
