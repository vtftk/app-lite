use anyhow::Context;
use sea_orm::{entity::prelude::*, sea_query::OnConflict, ActiveValue::Set, IntoActiveModel};
use serde::{Deserialize, Serialize};

use super::shared::DbResult;

// Type alias helpers for the database entity types
pub type KeyValueModel = Model;
pub type KeyValueEntity = Entity;
pub type KeyValueActiveModel = ActiveModel;
pub type KeyValueColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "key_value")]
pub struct Model {
    /// Key for the key value pair
    #[sea_orm(primary_key)]
    pub key: String,
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateKeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Default, Deserialize)]
#[allow(unused)]
pub struct UpdateKeyValue {
    pub value: Option<String>,
}

impl Model {
    /// Create a new sound
    pub async fn create<C>(db: &C, create: CreateKeyValue) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            key: Set(create.key.to_string()),
            value: Set(create.value),
        };

        Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(Column::Key)
                    .update_column(Column::Value)
                    .to_owned(),
            )
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_key(db, &create.key)
            .await?
            .context("model was not inserted")?;
        Ok(model)
    }

    /// Find a specific key value by key
    pub async fn get_by_key<C>(db: &C, key: &str) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(key).one(db).await
    }

    /// Find all key values
    #[allow(unused)]
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find().all(db).await
    }

    /// Update the current key value
    #[allow(unused)]
    pub async fn update<C>(self, db: &C, data: UpdateKeyValue) -> DbResult<Self>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut this = self.into_active_model();

        this.value = data.value.map(Set).unwrap_or(this.value);

        let this = this.update(db).await?;
        Ok(this)
    }
}
