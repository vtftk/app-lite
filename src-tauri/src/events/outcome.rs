use std::collections::HashSet;

use anyhow::{anyhow, Context};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    database::entity::{
        events::{
            BitsAmount, EventOutcome, EventOutcomeBits, EventOutcomePlaySound,
            EventOutcomeThrowable, EventOutcomeTriggerHotkey, ThrowableData,
        },
        items::ThrowableImageConfig,
        ItemModel, SoundModel,
    },
    state::app_data::{ItemWithImpactSoundIds, ThrowableConfig},
};

use super::{
    matching::{EventData, EventInputData},
    EventMessage,
};

/// Produce a message for an outcome
pub async fn produce_outcome_message(
    db: &DatabaseConnection,
    event_data: EventData,
    outcome: EventOutcome,
) -> anyhow::Result<EventMessage> {
    match outcome {
        EventOutcome::ThrowBits(data) => throw_bits_outcome(db, event_data, data).await,
        EventOutcome::Throwable(data) => throwable_outcome(db, data).await,
        EventOutcome::TriggerHotkey(data) => trigger_hotkey_outcome(data),
        EventOutcome::PlaySound(data) => play_sound_outcome(db, data).await,
    }
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

    let throwable_config = match bit_icon {
        Some(bit_icon) => create_throwable_config(db, &[bit_icon]).await?,
        None => create_default_bit_throwable(input),
    };

    let amount = match data.amount {
        BitsAmount::Dynamic { max_amount } => input.min(max_amount as i64) as u32,
        BitsAmount::Fixed { amount } => amount,
    };

    Ok(EventMessage::ThrowItem {
        config: throwable_config,
        amount,
    })
}

// Produce a throwable message
async fn throwable_outcome(
    db: &DatabaseConnection,
    data: EventOutcomeThrowable,
) -> anyhow::Result<EventMessage> {
    match data.data {
        ThrowableData::Throw {
            throwable_ids,
            amount,
        } => {
            let throwable_config = create_throwable_config(db, &throwable_ids).await?;

            Ok(EventMessage::ThrowItem {
                config: throwable_config,
                amount,
            })
        }
        ThrowableData::Barrage {
            throwable_ids,
            amount_per_throw,
            frequency,
            amount,
        } => {
            let throwable_config = create_throwable_config(db, &throwable_ids).await?;

            Ok(EventMessage::ThrowItemBarrage {
                config: throwable_config,
                amount,
                frequency,
                amount_per_throw,
            })
        }
    }
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
    let config = SoundModel::get_by_id(db, data.sound_id)
        .await?
        .context("sound config not found")?;

    Ok(EventMessage::PlaySound { config })
}

pub async fn create_throwable_config(
    db: &DatabaseConnection,
    item_ids: &[Uuid],
) -> anyhow::Result<ThrowableConfig> {
    let items: Vec<ItemWithImpactSoundIds> = ItemModel::get_by_ids_with_impact_sounds(db, item_ids)
        .await?
        .into_iter()
        .map(|(item, impact_sounds)| ItemWithImpactSoundIds {
            item,
            impact_sound_ids: impact_sounds
                .into_iter()
                .map(|impact_sound| impact_sound.sound_id)
                .collect(),
        })
        .collect();

    // Collect all unique impact sound IDs
    let impact_sound_ids: Vec<Uuid> = items
        .iter()
        .flat_map(|item| item.impact_sound_ids.iter())
        .cloned()
        // Collect to HashSet first for unique IDs
        .collect::<HashSet<Uuid>>()
        .into_iter()
        .collect::<Vec<Uuid>>();

    let impact_sounds = SoundModel::get_by_ids(db, &impact_sound_ids).await?;

    Ok(ThrowableConfig {
        items,
        impact_sounds,
    })
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

pub fn create_default_bit_throwable(amount: i64) -> ThrowableConfig {
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
    let impact_sounds: Vec<SoundModel> = DEFAULT_SOUND_FILES
        .iter()
        .map(|(name, file_name)| SoundModel {
            id: Uuid::new_v4(),
            name: name.to_string(),
            src: format!("backend://defaults/sounds/{file_name}"),
            volume: 1.,
            order: 0,
        })
        .collect();

    let impact_sound_ids: Vec<Uuid> = impact_sounds.iter().map(|sound| sound.id).collect();

    let item = ItemModel {
        id: Uuid::new_v4(),
        name: "<builtin-bits>".to_string(),
        image: ThrowableImageConfig {
            src: bit_src,
            pixelate: false,
            scale: 1.0,
            weight: 1.0,
        },
        order: 0,
    };

    let item = ItemWithImpactSoundIds {
        item,
        impact_sound_ids,
    };

    let items = vec![item];

    ThrowableConfig {
        items,
        impact_sounds,
    }
}
