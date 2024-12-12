//! # Key Value
//!
//! Key value store in the database

use anyhow::Context;
use sea_orm::{entity::prelude::*, sea_query::OnConflict, ActiveValue::Set};
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
    #[serde(rename = "type")]
    #[sea_orm(column_name = "type")]
    pub ty: KeyValueType,
    pub value: String,
}

/// Key value type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum KeyValueType {
    /// Plain text is stored
    #[sea_orm(string_value = "Text")]
    Text,
    /// Number is stored as plain text
    #[sea_orm(string_value = "Number")]
    Number,
    /// Object is stored as JSON
    #[sea_orm(string_value = "Object")]
    Object,
    /// Array is stored as JSON
    #[sea_orm(string_value = "Array")]
    Array,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateKeyValue {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub ty: KeyValueType,
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
            ty: Set(create.ty),
        };

        Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(Column::Key)
                    .update_column(Column::Value)
                    .update_column(Column::Ty)
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
}
