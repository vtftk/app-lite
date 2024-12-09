use std::{sync::Arc, time::Duration};

use anyhow::anyhow;
use futures::{future::BoxFuture, stream::FuturesUnordered, StreamExt};
use log::{debug, error};
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast;

use crate::{
    database::entity::{commands::CommandOutcome, EventModel},
    script::runtime::{CommandContext, CommandContextUser, ScriptExecutorHandle},
    twitch::manager::{TwitchEvent, TwitchManager},
};

use super::{
    event_processing::{assert_required_role, EventsStateShared},
    matching::{
        match_chat_event, match_cheer_bits_event, match_follow_event,
        match_gifted_subscription_event, match_re_subscription_event, match_redeem_event,
        match_subscription_event, CommandWithContext, EventData, EventInputData, EventMatchingData,
        ScriptWithContext,
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
    let events_state = EventsStateShared::default();

    while let Ok(event) = twitch_event_rx.recv().await {
        debug!("twitch event received: {:?}", event);

        tokio::spawn({
            let db = db.clone();
            let twitch_manager = twitch_manager.clone();
            let script_handle = script_handle.clone();
            let event_sender = event_sender.clone();
            let events_state = events_state.clone();

            async move {
                let result = process_twitch_event(
                    db,
                    twitch_manager,
                    script_handle,
                    event_sender,
                    events_state,
                    event,
                )
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
    events_state: EventsStateShared,
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
                    &script_handle,
                    &twitch_manager,
                    &events_state,
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
                    &events_state,
                    event,
                    match_data.event_data.clone(),
                ))
            });

    let mut futures = command_futures
        .chain(script_futures)
        .chain(event_futures)
        .collect::<FuturesUnordered<BoxFuture<'_, anyhow::Result<()>>>>();

    while let Some(result) = futures.next().await {
        if let Err(err) = result {
            error!("error while executing event outcome: {err:?}");
        }
    }

    Ok(())
}

pub async fn execute_command(
    script_handle: &ScriptExecutorHandle,
    twitch_manager: &Arc<TwitchManager>,
    events_state: &EventsStateShared,
    command: CommandWithContext,
    event_data: EventData,
) -> anyhow::Result<()> {
    let EventInputData::Chat { message, .. } = &event_data.input_data else {
        return Err(anyhow!("Non chat input data provided for chat execute"));
    };

    let user = match event_data.user {
        Some(value) => value,
        None => return Err(anyhow!("failed to get twitch user")),
    };

    // Ensure required role is present
    if !assert_required_role(
        twitch_manager,
        Some(user.id.clone()),
        &command.command.require_role,
    )
    .await
    {
        debug!("skipping event: missing required role");
        return Ok(());
    }

    // Ensure cooldown is not active
    if !events_state
        .is_cooldown_elapsed(
            &command.command.id,
            Duration::from_millis(command.command.cooldown as u64),
        )
        .await
    {
        debug!("skipping command: cooldown");
        return Ok(());
    }

    match command.command.outcome {
        CommandOutcome::Template { message: _message } => {
            // TODO: Not implemented yet
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

            script_handle.execute_command(script, ctx).await?;
        }
    }

    // Mark last execution for cooldown
    events_state.set_last_executed(&command.command.id).await;

    Ok(())
}

pub async fn execute_script(
    script_handle: &ScriptExecutorHandle,
    script: ScriptWithContext,
    event_data: EventData,
) -> anyhow::Result<()> {
    script_handle
        .execute(script.script.script, script.event, event_data)
        .await?;
    Ok(())
}

pub async fn execute_event(
    db: &DatabaseConnection,
    twitch_manager: &Arc<TwitchManager>,
    event_sender: &broadcast::Sender<EventMessage>,
    events_state: &EventsStateShared,
    event: EventModel,
    event_data: EventData,
) -> anyhow::Result<()> {
    // Ensure required role is present
    if !assert_required_role(
        twitch_manager,
        event_data.user.as_ref().map(|value| value.id.clone()),
        &event.require_role,
    )
    .await
    {
        debug!("skipping event: missing required role");
        return Ok(());
    }

    // Ensure cooldown is not active
    if !events_state
        .is_cooldown_elapsed(&event.id, Duration::from_millis(event.cooldown as u64))
        .await
    {
        debug!("skipping event: cooldown");
        return Ok(());
    }

    // Wait for outcome delay
    tokio::time::sleep(Duration::from_millis(event.outcome_delay as u64)).await;

    // Produce outcome message and send it
    let msg = produce_outcome_message(&db, event_data, event.outcome).await?;
    _ = event_sender.send(msg);

    // Update last execution time
    events_state.set_last_executed(&event.id).await;

    Ok(())
}
