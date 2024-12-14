use anyhow::Context;
use futures::{future::BoxFuture, stream::FuturesUnordered, TryStreamExt};
use sea_orm::{
    entity::prelude::*, ActiveValue::Set, FromJsonQueryResult, IntoActiveModel, QueryOrder,
    UpdateResult,
};
use serde::{Deserialize, Serialize};

use super::{
    event_executions::{EventExecutionColumn, EventExecutionModel},
    shared::{DbResult, MinMax, MinimumRequireRole},
};

// Type alias helpers for the database entity types
pub type EventModel = Model;
pub type EventEntity = Entity;
pub type EventActiveModel = ActiveModel;
pub type EventColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "events")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// Whether the event is enabled
    pub enabled: bool,
    /// Name of the event handler
    pub name: String,
    /// Duplicate of the "trigger" column but just the string key to allow querying
    /// derived from "trigger"
    #[serde(skip)]
    pub trigger_type: EventTriggerType,
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
    /// Ordering
    pub order: u32,
}

#[derive(Default, Deserialize)]
pub struct UpdateEventOrdering {
    pub id: Uuid,
    pub order: u32,
}

/// Copy of the [EventTrigger] enum but string variants to
/// support storing in the database as strings for querying
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum EventTriggerType {
    #[sea_orm(string_value = "Redeem")]
    Redeem,
    #[sea_orm(string_value = "Command")]
    Command,
    #[sea_orm(string_value = "Follow")]
    Follow,
    #[sea_orm(string_value = "Subscription")]
    Subscription,
    #[sea_orm(string_value = "GiftedSubscription")]
    GiftedSubscription,
    #[sea_orm(string_value = "Bits")]
    Bits,
    #[sea_orm(string_value = "Raid")]
    Raid,
}

impl EventTriggerType {
    pub fn from_event_trigger(trigger: &EventTrigger) -> Self {
        match trigger {
            EventTrigger::Redeem { .. } => EventTriggerType::Redeem,
            EventTrigger::Command { .. } => EventTriggerType::Command,
            EventTrigger::Follow => EventTriggerType::Follow,
            EventTrigger::Subscription => EventTriggerType::Subscription,
            EventTrigger::GiftedSubscription => EventTriggerType::GiftedSubscription,
            EventTrigger::Bits { .. } => EventTriggerType::Bits,
            EventTrigger::Raid { .. } => EventTriggerType::Raid,
        }
    }
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
        /// Amount of items to throw
        amount: i64,

        /// Override to derive amount of items to throw
        #[serde(default)]
        use_input_amount: bool,
        /// Additional configuration for when use_input_amount is true
        #[serde(default)]
        input_amount_config: InputAmountConfig,
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
        amount: i64,

        /// Override to derive amount of items to throw
        #[serde(default)]
        use_input_amount: bool,
        /// Additional configuration for when use_input_amount is true
        #[serde(default)]
        input_amount_config: InputAmountConfig,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputAmountConfig {
    /// Multiplier to apply against the input amount
    pub multiplier: f64,
    /// Allowed range for the input
    pub range: MinMax<i64>,
}

impl Default for InputAmountConfig {
    fn default() -> Self {
        Self {
            multiplier: 1.,
            range: MinMax { min: 1, max: 100 },
        }
    }
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
    Fixed { amount: i64 },

    /// Throw the number of bits the user provided
    Dynamic {
        /// Maximum number to throw
        max_amount: i64,
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
pub enum Relation {
    /// Event can have many executions
    #[sea_orm(has_many = "super::event_executions::Entity")]
    Executions,
}

impl Related<super::event_executions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Executions.def()
    }
}

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
    pub order: Option<u32>,
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
            trigger_type: Set(EventTriggerType::from_event_trigger(&create.trigger)),
            trigger: Set(create.trigger),
            outcome: Set(create.outcome),
            cooldown: Set(create.cooldown),
            require_role: Set(create.require_role),
            outcome_delay: Set(create.outcome_delay),
            order: Set(0),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, id)
            .await?
            .context("model was not inserted")?;

        Ok(model)
    }

    /// Find the most recent execution of this event
    pub async fn last_execution<C>(&self, db: &C) -> DbResult<Option<EventExecutionModel>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        self.find_related(super::event_executions::Entity)
            .order_by_desc(EventExecutionColumn::CreatedAt)
            .one(db)
            .await
    }

    /// Find a specific event by ID
    pub async fn get_by_id<C>(db: &C, id: Uuid) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    /// Find a specific event by a specific trigger type
    ///
    /// Filters to only events marked as enabled
    pub async fn get_by_trigger_type<C>(
        db: &C,
        trigger_type: EventTriggerType,
    ) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find()
            .filter(
                Column::TriggerType
                    .eq(trigger_type)
                    .and(Column::Enabled.eq(true)),
            )
            .all(db)
            .await
    }

    /// Find all events
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find().order_by_asc(Column::Order).all(db).await
    }

    /// Update the current event
    pub async fn update<C>(self, db: &C, data: UpdateEvent) -> DbResult<Self>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut this = self.into_active_model();

        this.enabled = data.enabled.map(Set).unwrap_or(this.enabled);
        this.name = data.name.map(Set).unwrap_or(this.name);
        this.trigger_type = data
            .trigger
            .as_ref()
            .map(EventTriggerType::from_event_trigger)
            .map(Set)
            .unwrap_or(this.trigger_type);
        this.trigger = data.trigger.map(Set).unwrap_or(this.trigger);
        this.outcome = data.outcome.map(Set).unwrap_or(this.outcome);
        this.cooldown = data.cooldown.map(Set).unwrap_or(this.cooldown);
        this.require_role = data.require_role.map(Set).unwrap_or(this.require_role);
        this.outcome_delay = data.outcome_delay.map(Set).unwrap_or(this.outcome_delay);
        this.order = data.order.map(Set).unwrap_or(this.order);

        let this = this.update(db).await?;
        Ok(this)
    }

    pub async fn update_order<C>(db: &C, data: Vec<UpdateEventOrdering>) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let _results: Result<Vec<UpdateResult>, DbErr> = data
            .into_iter()
            .map(|data| -> BoxFuture<'_, DbResult<UpdateResult>> {
                Box::pin(
                    Entity::update_many()
                        .filter(Column::Id.eq(data.id))
                        .col_expr(Column::Order, data.order.into())
                        .exec(db),
                )
            })
            .collect::<FuturesUnordered<BoxFuture<'_, DbResult<UpdateResult>>>>()
            .try_collect()
            .await;

        Ok(())
    }
}
