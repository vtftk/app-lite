use super::{
    matching::{EventData, EventInputData},
    EventMessage, ItemWithSoundIds, ItemsWithSounds, ThrowItemConfig, ThrowItemMessage,
};
use crate::{
    database::entity::{
        events::{
            EventModel, EventOutcome, EventOutcomeBits, EventOutcomeChannelEmotes,
            EventOutcomePlaySound, EventOutcomeSendChat, EventOutcomeThrowable,
            EventOutcomeTriggerHotkey, ThrowableAmountData,
        },
        items::{ItemConfig, ItemImageConfig, ItemModel},
        items_sounds::SoundType,
        sounds::{PartialSoundModel, SoundModel},
    },
    twitch::manager::Twitch,
};
use anyhow::{anyhow, Context};
use chrono::Utc;
use sea_orm::DatabaseConnection;
use std::collections::HashSet;
use twitch_api::types::SubscriptionTier;
use uuid::Uuid;

/// Produce a message for an outcome
pub async fn produce_outcome_message(
    db: &DatabaseConnection,
    twitch: &Twitch,

    event: EventModel,
    event_data: EventData,
) -> anyhow::Result<Option<EventMessage>> {
    match event.outcome {
        EventOutcome::ThrowBits(data) => throw_bits_outcome(db, event_data, data).await.map(Some),
        EventOutcome::Throwable(data) => throwable_outcome(db, event_data, data).await.map(Some),
        EventOutcome::TriggerHotkey(data) => trigger_hotkey_outcome(data).map(Some),
        EventOutcome::PlaySound(data) => play_sound_outcome(db, data).await.map(Some),
        EventOutcome::SendChatMessage(data) => {
            send_chat_message(twitch, event_data, data).await?;
            Ok(None)
        }
        EventOutcome::ChannelEmotes(data) => throw_channel_emotes_outcome(twitch, event_data, data)
            .await
            .map(Some),
    }
}

fn format_subscription_tier(tier: SubscriptionTier) -> &'static str {
    match tier {
        SubscriptionTier::Tier1 => "Tier 1",
        SubscriptionTier::Tier2 => "Tier 2",
        SubscriptionTier::Tier3 => "Tier 3",
        SubscriptionTier::Prime => "Prime",
        SubscriptionTier::Other(_) => "Other",
    }
}

async fn send_chat_message(
    twitch: &Twitch,
    event_data: EventData,
    data: EventOutcomeSendChat,
) -> anyhow::Result<()> {
    let mut message = data.template;

    let user_name = event_data
        .user
        .map(|user| user.name.to_string())
        .unwrap_or_else(|| "Anonymous".to_string());

    message = message.replace("$(user)", user_name.as_str());

    match event_data.input_data {
        EventInputData::Redeem {
            reward_name,
            cost,
            user_input,
            ..
        } => {
            message = message.replace("$(userInput)", user_input.as_str());
            message = message.replace("$(rewardName)", reward_name.as_str());
            message = message.replace("$(rewardCost)", cost.to_string().as_str());
        }
        EventInputData::Bits {
            bits,
            message: user_input,
            ..
        } => {
            message = message.replace("$(userInput)", user_input.to_string().as_str());
            message = message.replace("$(bits)", bits.to_string().as_str());
        }
        EventInputData::AdBreakBegin { duration_seconds } => {
            message = message.replace("$(duration)", duration_seconds.to_string().as_str());
        }
        EventInputData::Subscription { tier, .. } => {
            message = message.replace("$(tier)", format_subscription_tier(tier));
        }
        EventInputData::GiftedSubscription { total, tier, .. } => {
            message = message.replace("$(tier)", format_subscription_tier(tier));
            message = message.replace("$(total)", total.to_string().as_str());
        }
        EventInputData::ReSubscription {
            cumulative_months,
            duration_months,
            message: user_input,
            tier,
            ..
        } => {
            message = message.replace("$(tier)", format_subscription_tier(tier));
            message = message.replace("$(userInput)", user_input.as_str());
            message = message.replace(
                "$(cumulativeMonths)",
                cumulative_months.to_string().as_str(),
            );
            message = message.replace("$(durationMonths)", duration_months.to_string().as_str());
        }
        EventInputData::Chat {
            message: user_input,
            ..
        } => {
            message = message.replace("$(userInput)", user_input.as_str());
        }
        EventInputData::Raid { viewers } => {
            message = message.replace("$(viewers)", viewers.to_string().as_str());
        }
        EventInputData::ShoutoutReceive { viewer_count } => {
            message = message.replace("$(viewers)", viewer_count.to_string().as_str());
        }
        EventInputData::None => {}
    }

    if message.len() < 500 {
        twitch.send_chat_message(&message).await?;
    } else {
        let mut chars = message.chars();

        loop {
            let message = chars.by_ref().take(500).collect::<String>();
            if message.is_empty() {
                break;
            }

            twitch.send_chat_message(&message).await?;
        }
    }

    Ok(())
}

/// Produce a bits throwing outcome message
async fn throw_bits_outcome(
    db: &DatabaseConnection,
    event_data: EventData,
    data: EventOutcomeBits,
) -> anyhow::Result<EventMessage> {
    let input = match event_data.input_data {
        EventInputData::Bits { bits, .. } => bits,
        _ => {
            return Err(anyhow!(
                "unexpected event input, throw bits requires bit count"
            ))
        }
    };

    let sets = [data._1, data._100, data._1000, data._5000, data._10000];
    let mut bit_index: usize = match input {
        1..=99 => 0,
        100..=999 => 1,
        1000..=4999 => 2,
        5000..=9999 => 3,
        _ => 4,
    };

    let mut bit_icon: Option<Uuid> = None;

    // Go through the bit icons till we find one
    while bit_icon.is_none() {
        bit_icon = sets.get(bit_index).and_then(|value| *value);

        // Move to index before
        match bit_index.checked_sub(1) {
            Some(value) => {
                bit_index = value;
            }
            None => break,
        }
    }

    let items = match bit_icon {
        Some(bit_icon) => resolve_items(db, &[bit_icon]).await?,
        None => create_default_bit_throwable(input),
    };

    create_throwable_message(items, data.amount, Some(input))
}

/// Produce a channel emote throwing outcome message
async fn throw_channel_emotes_outcome(
    twitch: &Twitch,
    event_data: EventData,
    data: EventOutcomeChannelEmotes,
) -> anyhow::Result<EventMessage> {
    let user = match event_data.user {
        Some(user) => user,
        None => {
            return Err(anyhow!(
                "cannot throw channel emotes when user is not present"
            ))
        }
    };

    let emotes = twitch.get_channel_emotes(user.id.clone()).await?;

    // Create sounds from builtins
    let sounds: Vec<PartialSoundModel> = create_default_impact_sounds();
    let impact_sound_ids: Vec<Uuid> = sounds.iter().map(|sound| sound.id).collect();

    let items = emotes
        .into_iter()
        .map(|emote| {
            let item = ItemModel {
                id: Uuid::new_v4(),
                name: "<builtin-bits>".to_string(),
                config: ItemConfig {
                    image: ItemImageConfig {
                        src: emote.images.url_4x,
                        pixelate: false,
                        scale: 1.0,
                        weight: 1.0,
                    },
                    windup: Default::default(),
                },
                order: 0,
                created_at: Utc::now(),
            };

            ItemWithSoundIds {
                item,
                impact_sound_ids: impact_sound_ids.clone(),
                windup_sound_ids: Vec::new(),
            }
        })
        .collect();

    let items = ItemsWithSounds { items, sounds };

    create_throwable_message(items, data.amount, None)
}

fn get_event_data_input_amount(event_data: &EventData) -> Option<i64> {
    // Compute amount derived from input
    match &event_data.input_data {
        EventInputData::Bits { bits, .. } => Some(*bits),
        EventInputData::GiftedSubscription { total, .. } => Some(*total),
        EventInputData::Subscription { .. } => Some(1),
        EventInputData::ReSubscription {
            cumulative_months, ..
        } => Some(*cumulative_months),
        EventInputData::Chat { cheer, .. } => cheer.map(|value| value as i64),
        EventInputData::Raid { viewers } => Some(*viewers),

        _ => None,
    }
}

fn create_throwable_message(
    items: ItemsWithSounds,
    amount: ThrowableAmountData,
    input_amount: Option<i64>,
) -> anyhow::Result<EventMessage> {
    match amount {
        ThrowableAmountData::Throw {
            amount,
            use_input_amount,
            input_amount_config,
        } => {
            let amount = if use_input_amount {
                let input_amount = input_amount.unwrap_or(amount);

                // Apply multiplier
                let input_amount =
                    (input_amount as f64 * input_amount_config.multiplier).floor() as i64;

                // Clamp within allowed range

                input_amount.clamp(input_amount_config.range.min, input_amount_config.range.max)
            } else {
                amount
            };

            Ok(EventMessage::ThrowItem(ThrowItemMessage {
                items,
                config: ThrowItemConfig::All { amount },
            }))
        }
        ThrowableAmountData::Barrage {
            amount_per_throw,
            frequency,
            amount,
            use_input_amount,
            input_amount_config,
        } => {
            let amount = if use_input_amount {
                let input_amount = input_amount.unwrap_or(amount);

                // Apply multiplier
                let input_amount =
                    (input_amount as f64 * input_amount_config.multiplier).floor() as i64;

                // Clamp within allowed range

                input_amount.clamp(input_amount_config.range.min, input_amount_config.range.max)
            } else {
                amount
            };

            Ok(EventMessage::ThrowItem(ThrowItemMessage {
                items,
                config: ThrowItemConfig::Barrage {
                    amount_per_throw,
                    amount,
                    frequency,
                },
            }))
        }
    }
}

// Produce a throwable message
async fn throwable_outcome(
    db: &DatabaseConnection,
    event_data: EventData,
    data: EventOutcomeThrowable,
) -> anyhow::Result<EventMessage> {
    let items = resolve_items(db, &data.throwable_ids).await?;

    create_throwable_message(items, data.amount, get_event_data_input_amount(&event_data))
}

/// Produce a hotkey trigger message
fn trigger_hotkey_outcome(data: EventOutcomeTriggerHotkey) -> anyhow::Result<EventMessage> {
    Ok(EventMessage::TriggerHotkey {
        hotkey_id: data.hotkey_id,
    })
}

/// Produce a sound outcome event message
async fn play_sound_outcome(
    db: &DatabaseConnection,
    data: EventOutcomePlaySound,
) -> anyhow::Result<EventMessage> {
    let config = SoundModel::get_by_id_partial(db, data.sound_id)
        .await?
        .context("sound config not found")?;

    Ok(EventMessage::PlaySound { config })
}

pub async fn resolve_items(
    db: &DatabaseConnection,
    item_ids: &[Uuid],
) -> anyhow::Result<ItemsWithSounds> {
    let mut sound_ids = HashSet::new();

    let items: Vec<ItemWithSoundIds> = ItemModel::get_by_ids_with_sounds(db, item_ids)
        .await?
        .into_iter()
        .map(|(item, sounds)| {
            let mut impact_sound_ids = Vec::new();
            let mut windup_sound_ids = Vec::new();

            for sound in sounds {
                sound_ids.insert(sound.sound_id);

                match sound.sound_type {
                    SoundType::Impact => impact_sound_ids.push(sound.sound_id),
                    SoundType::Windup => windup_sound_ids.push(sound.sound_id),
                }
            }

            ItemWithSoundIds {
                item,
                impact_sound_ids,
                windup_sound_ids,
            }
        })
        .collect();

    // Collect all unique sound IDs
    let sound_ids: Vec<Uuid> = sound_ids.into_iter().collect::<Vec<Uuid>>();

    let sounds = SoundModel::get_by_ids_partial(db, &sound_ids).await?;

    Ok(ItemsWithSounds { items, sounds })
}

// Default sound file names
#[rustfmt::skip]
const DEFAULT_SOUND_FILES: &[(&str, &str)] = &[
    ("Seq 2.1 Hit #1 96 HK1", "Seq_2_1_Hit_1_96_HK1.wav"),
    ("Seq 2.1 Hit #2 96 HK1", "Seq_2_1_Hit_2_96_HK1.wav"),
    ("Seq 2.1 Hit #3 96 HK1", "Seq_2_1_Hit_3_96_HK1.wav"),
    ("Seq 2.27 Hit #1 96 HK1", "Seq_2_27_Hit_1_96_HK1.wav"),
    ("Seq 2.27 Hit #2 96 HK1", "Seq_2_27_Hit_2_96_HK1.wav"),
    ("Seq 2.27 Hit #3 96 HK1", "Seq_2_27_Hit_3_96_HK1.wav"),
    ("Seq1.15 Hit #1 96 HK1", "Seq1_15_Hit_1_96_HK1.wav"),
    ("Seq1.15 Hit #2 96 HK1", "Seq1_15_Hit_2_96_HK1.wav"),
    ("Seq1.15 Hit #3 96 HK1", "Seq1_15_Hit_3_96_HK1.wav"),
];

fn create_default_impact_sounds() -> Vec<PartialSoundModel> {
    DEFAULT_SOUND_FILES
        .iter()
        .map(|(_name, file_name)| PartialSoundModel {
            id: Uuid::new_v4(),
            src: format!("backend://defaults/sounds/{file_name}"),
            volume: 1.,
        })
        .collect()
}

pub fn create_default_bit_throwable(amount: i64) -> ItemsWithSounds {
    // Get the general bit category
    let bit_index: usize = match amount {
        1..=99 => 0,
        100..=999 => 1,
        1000..=4999 => 2,
        5000..=9999 => 3,
        _ => 4,
    };

    let bit_file_name = match bit_index {
        0 => "1.png",
        1 => "100.png",
        2 => "1000.png",
        3 => "5000.png",
        _ => "10000.png",
    };

    let bit_src = format!("backend://defaults/bits/{bit_file_name}");

    // Create sounds from builtins
    let impact_sounds: Vec<PartialSoundModel> = create_default_impact_sounds();
    let impact_sound_ids: Vec<Uuid> = impact_sounds.iter().map(|sound| sound.id).collect();

    let item = ItemModel {
        id: Uuid::new_v4(),
        name: "<builtin-bits>".to_string(),
        config: ItemConfig {
            image: ItemImageConfig {
                src: bit_src,
                pixelate: false,
                scale: 1.0,
                weight: 1.0,
            },
            windup: Default::default(),
        },
        order: 0,
        created_at: Utc::now(),
    };

    let item = ItemWithSoundIds {
        item,
        impact_sound_ids,
        windup_sound_ids: Vec::new(),
    };

    let items = vec![item];

    ItemsWithSounds {
        items,
        sounds: impact_sounds,
    }
}
