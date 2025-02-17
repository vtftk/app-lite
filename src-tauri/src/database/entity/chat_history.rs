use super::shared::DbResult;
use sea_orm::{
    entity::prelude::*, sea_query::Func, ActiveValue::Set, FromQueryResult, QuerySelect,
};
use serde::{Deserialize, Serialize};
use twitch_api::types::UserId;

// Type alias helpers for the database entity types
pub type ChatHistoryModel = Model;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_history")]
pub struct Model {
    /// Unique ID of the log
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// ID of the twitch user
    pub user_id: String,
    /// Chat message data
    pub message: String,
    /// Optional cheer amount
    pub cheer: Option<u32>,
    /// Creation time of the chat message
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug)]
pub struct CreateChatHistory {
    /// Unique ID of the log
    pub id: Uuid,
    /// ID of the twitch user
    pub user_id: String,
    /// Chat message data
    pub message: String,
    /// Optional cheer amount
    pub cheer: Option<u32>,
    /// Creation time of the chat message
    pub created_at: DateTimeUtc,
}

impl Model {
    /// Create a new script
    pub async fn create<C>(db: &C, create: CreateChatHistory) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            id: Set(create.id),
            user_id: Set(create.user_id),
            message: Set(create.message),
            cheer: Set(create.cheer),
            created_at: Set(create.created_at),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        Ok(())
    }

    /// Estimates the size in bytes that the current chat history is taking up
    pub async fn estimate_size<C>(db: &C) -> DbResult<u32>
    where
        C: ConnectionTrait + Send + 'static,
    {
        #[derive(Default, FromQueryResult)]
        struct PartialModel {
            total_message_length: Option<u32>,
        }

        let result = Entity::find()
            .expr_as(
                Func::sum(Func::char_length(Expr::col(Column::Message))),
                "total_message_length",
            )
            .into_model::<PartialModel>()
            .one(db)
            .await?;

        let result = match result {
            Some(result) => result,
            None => return Ok(0),
        };

        Ok(result.total_message_length.unwrap_or_default())
    }

    pub async fn count_since<C>(
        db: &C,
        start_date: DateTimeUtc,
        exclude_id: Option<UserId>,
    ) -> DbResult<u64>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut select = Entity::find().filter(Column::CreatedAt.gt(start_date));

        if let Some(exclude_id) = exclude_id {
            select = select.filter(Column::UserId.ne(exclude_id.as_str()));
        }

        select.count(db).await
    }

    pub async fn delete_before<C>(db: &C, start_date: DateTimeUtc) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::delete_many()
            .filter(Column::CreatedAt.lt(start_date))
            .exec(db)
            .await?;
        Ok(())
    }
}
