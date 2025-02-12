use crate::{
    database::entity::{
        event_executions::{CreateEventExecution, EventExecutionMetadata, EventExecutionModel},
        events::EventModel,
        shared::MinimumRequireRole,
    },
    events::{
        matching::{
            match_ad_break_event, match_chat_event, match_cheer_bits_event, match_follow_event,
            match_gifted_subscription_event, match_raid_event, match_re_subscription_event,
            match_redeem_event, match_shoutout_receive_event, match_subscription_event, EventData,
            EventMatchingData,
        },
        outcome::produce_outcome_message,
        EventMessage,
    },
    twitch::{
        manager::Twitch,
        models::{TwitchEvent, TwitchEventUser},
    },
};
use anyhow::Context;
use chrono::TimeDelta;
use futures::{future::BoxFuture, stream::FuturesUnordered};
use log::{debug, error};
use sea_orm::{prelude::DateTimeUtc, sqlx::types::chrono::Utc, DatabaseConnection};
use std::time::Duration;
use tokio::{sync::broadcast, try_join};
use twitch_api::types::UserId;

pub async fn process_twitch_events(
    db: DatabaseConnection,
    twitch: Twitch,
    event_sender: broadcast::Sender<EventMessage>,

    mut twitch_event_rx: broadcast::Receiver<TwitchEvent>,
) {
    while let Ok(event) = twitch_event_rx.recv().await {
        debug!("twitch event received: {:?}", event);

        tokio::spawn({
            let db = db.clone();
            let twitch = twitch.clone();
            let event_sender = event_sender.clone();

            async move {
                let result = process_twitch_event(db, twitch, event_sender, event).await;

                if let Err(err) = result {
                    debug!("failed to process twitch event: {err:?}",);
                }
            }
        });
    }
}

async fn process_twitch_event(
    db: DatabaseConnection,
    twitch: Twitch,
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
        TwitchEvent::AdBreakBegin(event) => match_ad_break_event(&db, event).await?,
        TwitchEvent::ShoutoutReceive(event) => match_shoutout_receive_event(&db, event).await?,

        // Internal events
        TwitchEvent::ModeratorsChanged => {
            debug!("reloading mods list");
            twitch.load_moderator_list().await?;
            return Ok(());
        }
        TwitchEvent::VipsChanged => {
            debug!("reloading vips list");
            twitch.load_vip_list().await?;
            return Ok(());
        }
        TwitchEvent::RewardsChanged => {
            debug!("reloading rewards list");
            twitch.load_rewards_list().await?;
            return Ok(());
        }
        TwitchEvent::Reset => {
            debug!("resetting twitch manager");
            twitch.reset().await;
            return Ok(());
        }
    };

    let event_futures =
        match_data
            .events
            .into_iter()
            .map(|event| -> BoxFuture<'_, anyhow::Result<()>> {
                Box::pin(execute_event(
                    &db,
                    &twitch,
                    &event_sender,
                    event,
                    match_data.event_data.clone(),
                ))
            });

    let mut futures =
        event_futures.collect::<FuturesUnordered<BoxFuture<'_, anyhow::Result<()>>>>();

    use futures::StreamExt;

    while let Some(result) = futures.next().await {
        if let Err(err) = result {
            error!("error while executing event outcome: {err:?}");
        }
    }

    Ok(())
}

pub fn is_cooldown_elapsed(
    execution_time: DateTimeUtc,
    current_time: DateTimeUtc,
    cooldown: u32,
) -> anyhow::Result<bool> {
    let cooldown_end_time = execution_time
        .checked_add_signed(TimeDelta::milliseconds(cooldown as i64))
        .context("cooldown finishes too far in the future to compute")?;

    Ok(current_time > cooldown_end_time)
}

pub async fn is_event_cooldown_elapsed(
    db: &DatabaseConnection,
    event: &EventModel,
    user: Option<&TwitchEventUser>,
    current_time: DateTimeUtc,
) -> anyhow::Result<bool> {
    let cooldown = &event.cooldown;

    // No cooldown enabled
    if !cooldown.enabled {
        return Ok(true);
    }

    // Handle global cooldown (Check last execution)
    if !cooldown.per_user {
        let last_execution = event
            .last_execution(db, 0)
            .await
            .context("failed to request last execution for event")?;

        let last_execution = match last_execution {
            Some(value) => value,
            None => return Ok(true),
        };

        return is_cooldown_elapsed(last_execution.created_at, current_time, cooldown.duration);
    }

    let user = match user {
        Some(user) => user,
        // Anonymous users bypass the cooldown
        None => return Ok(true),
    };

    let mut offset = 0;

    loop {
        let last_execution = event
            .last_execution(db, offset)
            .await
            .context("failed to request last execution for event")?;

        // Cooldown elapsed if no more to check
        let last_execution = match last_execution {
            Some(value) => value,
            None => return Ok(true),
        };

        // Check if the execution has elapsed the cooldown
        let last_execution_elapsed =
            is_cooldown_elapsed(last_execution.created_at, current_time, cooldown.duration)?;

        // Execution was not from the target user
        if last_execution
            .metadata
            .user
            .is_none_or(|event_user| event_user.id != user.id)
        {
            offset += 1;

            // If the execution had elapsed the cooldown we can treat the rest as elapsed too (We can skip checking the rest)
            if last_execution_elapsed {
                return Ok(true);
            }

            continue;
        }

        // Found an execution
        return Ok(last_execution_elapsed);
    }
}

pub async fn execute_event(
    db: &DatabaseConnection,
    twitch: &Twitch,

    event_sender: &broadcast::Sender<EventMessage>,
    event: EventModel,
    event_data: EventData,
) -> anyhow::Result<()> {
    // Ensure required role is present
    if !has_required_role(
        twitch,
        event_data.user.as_ref().map(|value| value.id.clone()),
        &event.require_role,
    )
    .await
    {
        debug!("skipping event: missing required role");
        return Ok(());
    }

    let current_time = Utc::now();

    // Ensure cooldown is not active
    if !is_event_cooldown_elapsed(db, &event, event_data.user.as_ref(), current_time).await? {
        debug!("skipping event: cooldown");
        return Ok(());
    }

    // Create metadata for storage
    let metadata = EventExecutionMetadata {
        user: event_data.user.clone(),
        data: vec![(
            "input_data".to_string(),
            serde_json::to_value(&event_data.input_data)
                .context("failed to serialize event metadata")?,
        )],
    };

    // Wait for outcome delay
    tokio::time::sleep(Duration::from_millis(event.outcome_delay as u64)).await;

    let event_id = event.id;

    // Produce outcome message and send it
    if let Some(msg) = produce_outcome_message(db, twitch, event, event_data).await? {
        _ = event_sender.send(msg);
    }

    // Store event execution
    EventExecutionModel::create(
        db,
        CreateEventExecution {
            event_id,
            created_at: current_time,
            metadata,
        },
    )
    .await
    .context("failed to store last event execution")?;

    Ok(())
}

pub async fn has_required_role(
    twitch: &Twitch,
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
    if twitch
        .get_user_token()
        .await
        .is_some_and(|value| value.user_id == user)
    {
        return true;
    }

    // Check VIP and moderator lists
    if let MinimumRequireRole::Vip = required_role {
        return try_join!(twitch.get_vip_list(), twitch.get_moderator_list()).is_ok_and(
            |(vips, mods)| {
                vips.iter().any(|vip| vip.user_id == user)
                    || mods.iter().any(|mods| mods.user_id == user)
            },
        );
    }

    // Check just moderator list
    if let MinimumRequireRole::Mod = required_role {
        return twitch
            .get_moderator_list()
            .await
            .is_ok_and(|mods| mods.iter().any(|mods| mods.user_id == user));
    }

    // Check the user is a follower
    if let MinimumRequireRole::Follower = required_role {
        return twitch
            .get_follower_by_id(user)
            .await
            .is_ok_and(|value| value.is_some());
    }

    false
}
