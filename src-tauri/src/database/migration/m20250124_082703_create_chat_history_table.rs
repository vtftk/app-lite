use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChatHistory::Table)
                    .if_not_exists()
                    .col(pk_uuid(ChatHistory::Id))
                    .col(string(ChatHistory::UserId))
                    .col(string(ChatHistory::Message))
                    .col(integer_null(ChatHistory::Cheer))
                    .col(date_time(ChatHistory::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChatHistory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ChatHistory {
    Table,
    /// Twitch message ID
    Id,
    /// Twitch user ID
    UserId,
    /// Twitch message
    Message,
    /// Associated cheer amount (Optional)
    Cheer,
    /// Creation time of the chat message
    CreatedAt,
}
