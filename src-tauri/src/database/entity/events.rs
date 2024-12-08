use anyhow::Context;
use sea_orm::{entity::prelude::*, ActiveValue::Set, FromJsonQueryResult, IntoActiveModel};
use serde::{Deserialize, Serialize};

use super::shared::{DbResult, MinimumRequireRole};

// Type alias helpers for the database entity types
pub type EventModel = Model;
pub type EventEntity = Entity;
pub type EventActiveModel = ActiveModel;
pub type EventColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "events")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// Whether the event is enabled
    pub enabled: bool,
    /// Name of the event handler
    pub name: String,
    /// Input that should trigger the event
    pub trigger: EventTrigger,
    /// Outcome the event should trigger
    pub outcome: EventOutcome,
    /// Cooldown between each trigger of the even
    pub cooldown: u32,
    /// Minimum required role to trigger the event
    pub require_role: MinimumRequireRole,
    /// Delay before executing the outcome
    pub outcome_delay: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(tag = "type")]
pub enum EventTrigger {
    /// Redeem was triggered
    Redeem {
        /// ID of the reward required
        reward_id: String,
    },
    /// Command was sent
    Command {
        /// Command message required
        message: String,
    },
    /// User followed
    Follow,
    /// User subscribed
    Subscription,
    /// User gifted subscription
    GiftedSubscription,
    /// User gifts bits
    Bits {
        /// Minimum bits to trigger the event
        min_bits: u32,
    },
    /// Channel has been raided
    Raid {
        /// Minimum raiders required to trigger
        min_raiders: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ThrowableData {
    /// Throw items (All at once)
    Throw {
        /// IDs of the items that can be thrown
        throwable_ids: Vec<Uuid>,
        /// Amount to throw
        amount: u32,
    },

    /// Throw a throwable barrage
    Barrage {
        /// IDs of the items that can be thrown
        throwable_ids: Vec<Uuid>,
        /// Amount to throw for each throw
        amount_per_throw: u32,
        /// Time between each thrown item (Milliseconds)
        frequency: u32,
        /// Total amount of items to throw
        amount: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventOutcomeBits {
    /// Throwable to throw for 1 bit (Override, defaults to builtin)
    pub _1: Option<Uuid>,
    /// Throwable to throw for 100 bits (Override, defaults to builtin)
    pub _100: Option<Uuid>,
    /// Throwable to throw for 1000 bits (Override, defaults to builtin)
    pub _1000: Option<Uuid>,
    /// Throwable to throw for 5000 bits (Override, defaults to builtin)
    pub _5000: Option<Uuid>,
    /// Throwable to throw for 10000 bits (Override, defaults to builtin)
    pub _10000: Option<Uuid>,
    /// How many bits to throw
    pub amount: BitsAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum BitsAmount {
    /// Throw fixed amount of bits
    Fixed { amount: u32 },

    /// Throw the number of bits the user provided
    Dynamic {
        /// Maximum number to throw
        max_amount: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventOutcomeThrowable {
    /// Throwable data
    pub data: ThrowableData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventOutcomeTriggerHotkey {
    pub hotkey_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventOutcomePlaySound {
    pub sound_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromJsonQueryResult)]
#[serde(tag = "type")]
pub enum EventOutcome {
    /// Throw bits (Only compatible with bits trigger)
    ThrowBits(EventOutcomeBits),
    /// Throw something
    Throwable(EventOutcomeThrowable),
    /// Trigger a VTube studio hotkey
    TriggerHotkey(EventOutcomeTriggerHotkey),
    /// Trigger a sound
    PlaySound(EventOutcomePlaySound),
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateEvent {
    pub enabled: bool,
    pub name: String,
    pub trigger: EventTrigger,
    pub outcome: EventOutcome,
    pub cooldown: u32,
    pub require_role: MinimumRequireRole,
    pub outcome_delay: u32,
}

#[derive(Default, Deserialize)]
pub struct UpdateEvent {
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub trigger: Option<EventTrigger>,
    pub outcome: Option<EventOutcome>,
    pub cooldown: Option<u32>,
    pub require_role: Option<MinimumRequireRole>,
    pub outcome_delay: Option<u32>,
}

impl Model {
    /// Create a new event
    pub async fn create<C>(db: &C, create: CreateEvent) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            enabled: Set(create.enabled),
            name: Set(create.name),
            trigger: Set(create.trigger),
            outcome: Set(create.outcome),
            cooldown: Set(create.cooldown),
            require_role: Set(create.require_role),
            outcome_delay: Set(create.outcome_delay),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, id)
            .await?
            .context("model was not inserted")?;

        Ok(model)
    }

    /// Find a specific event by ID
    pub async fn get_by_id<C>(db: &C, id: Uuid) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    /// Find all events
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find().all(db).await
    }

    /// Update the current event
    pub async fn update<C>(self, db: &C, data: UpdateEvent) -> DbResult<Self>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut this = self.into_active_model();

        this.enabled = data.enabled.map(Set).unwrap_or(this.enabled);
        this.name = data.name.map(Set).unwrap_or(this.name);
        this.trigger = data.trigger.map(Set).unwrap_or(this.trigger);
        this.outcome = data.outcome.map(Set).unwrap_or(this.outcome);
        this.cooldown = data.cooldown.map(Set).unwrap_or(this.cooldown);
        this.require_role = data.require_role.map(Set).unwrap_or(this.require_role);
        this.outcome_delay = data.outcome_delay.map(Set).unwrap_or(this.outcome_delay);

        let this = this.update(db).await?;
        Ok(this)
    }
}
