//! # Event Executions Table
//!
//! Table that tracks all executions of an event and metadata
//! about the event that was executed

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
                    .table(EventExecutions::Table)
                    .if_not_exists()
                    .col(pk_uuid(EventExecutions::Id))
                    .col(uuid(EventExecutions::EventId))
                    .col(json(EventExecutions::Metadata))
                    .col(date_time(EventExecutions::CreatedAt))
                    // Connect to events table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_executions_event_id")
                            .from(EventExecutions::Table, EventExecutions::EventId)
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
            .drop_table(Table::drop().table(EventExecutions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum EventExecutions {
    Table,
    Id,
    EventId,
    Metadata,
    CreatedAt,
}
