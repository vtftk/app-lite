use sea_orm_migration::{prelude::*, schema::*};

use super::m20241208_060138_create_events_table::Events;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EventLogs::Table)
                    .if_not_exists()
                    .col(pk_uuid(EventLogs::Id))
                    .col(uuid(EventLogs::EventId))
                    .col(integer(EventLogs::Level))
                    .col(string(EventLogs::Message))
                    .col(date_time(EventLogs::CreatedAt))
                    // Connect to scripts table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_logs_event_id")
                            .from(EventLogs::Table, EventLogs::EventId)
                            .to(Events::Table, Events::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EventLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum EventLogs {
    Table,
    Id,
    EventId,
    Level,
    Message,
    CreatedAt,
}
