use super::shared::{DbResult, UpdateOrdering};
use anyhow::Context;
use chrono::Utc;
use futures::{future::BoxFuture, stream::FuturesUnordered, TryStreamExt};
use sea_orm::{
    entity::prelude::*, sea_query::Func, ActiveValue::Set, IntoActiveModel, QueryOrder,
    UpdateResult,
};
use serde::{Deserialize, Serialize};

// Type alias helpers for the database entity types
pub type SoundModel = Model;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sounds")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// Name of the sound
    pub name: String,
    /// Src URL for the image
    pub src: String,
    /// Volume of the sound 0-1
    pub volume: f32,
    /// Ordering
    pub order: u32,
    // Date time of creation
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Item can have many impact sounds
    #[sea_orm(has_many = "super::items_impact_sounds::Entity")]
    ImpactSounds,
}

impl Related<super::items_impact_sounds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImpactSounds.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateSound {
    pub name: String,
    pub src: String,
    pub volume: f32,
}

#[derive(Default, Deserialize)]
pub struct UpdateSound {
    pub name: Option<String>,
    pub src: Option<String>,
    pub volume: Option<f32>,
    pub order: Option<u32>,
}

impl Model {
    /// Create a new sound
    pub async fn create<C>(db: &C, create: CreateSound) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            name: Set(create.name),
            src: Set(create.src),
            volume: Set(create.volume),
            order: Set(0),
            created_at: Set(Utc::now()),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, id)
            .await?
            .context("model was not inserted")?;
        Ok(model)
    }

    /// Find a specific sound by ID
    pub async fn get_by_id<C>(db: &C, id: Uuid) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    /// Find sounds with IDs present in the provided list
    pub async fn get_by_ids<C>(db: &C, ids: &[Uuid]) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find()
            .filter(Column::Id.is_in(ids.iter().copied()))
            .all(db)
            .await
    }

    /// Find all sounds
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find()
            .order_by_asc(Column::Order)
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// Find all sounds with a matching name, optionally ignoring case
    pub async fn get_by_names<C>(db: &C, names: &[String], ignore_case: bool) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut select = Entity::find();

        if ignore_case {
            select = select.filter(
                // Convert stored name to lower case
                Expr::expr(Func::lower(Expr::col(Column::Name)))
                    // Compare with lowercase value
                    .is_in(names.iter().map(|name| name.to_lowercase())),
            )
        } else {
            select = select.filter(Column::Name.is_in(names))
        }

        select
            .order_by_asc(Column::Order)
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// Update the current sound
    pub async fn update<C>(self, db: &C, data: UpdateSound) -> DbResult<Self>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut this = self.into_active_model();

        if let Some(name) = data.name {
            this.name = Set(name);
        }

        if let Some(src) = data.src {
            this.src = Set(src);
        }

        if let Some(volume) = data.volume {
            this.volume = Set(volume);
        }
        this.order = data.order.map(Set).unwrap_or(this.order);

        let this = this.update(db).await?;
        Ok(this)
    }

    pub async fn update_order<C>(db: &C, data: Vec<UpdateOrdering>) -> DbResult<()>
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
