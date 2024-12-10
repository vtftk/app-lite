//! # Command Executions Table
//!
//! Table that tracks all executions of a command and metadata
//! about the command that was executed

use sea_orm_migration::{prelude::*, schema::*};

use super::m20241208_060200_create_commands_table::Commands;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CommandExecutions::Table)
                    .if_not_exists()
                    .col(pk_uuid(CommandExecutions::Id))
                    .col(uuid(CommandExecutions::CommandId))
                    .col(json(CommandExecutions::Metadata))
                    .col(date_time(CommandExecutions::CreatedAt))
                    // Connect to events table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_command_executions_event_id")
                            .from(CommandExecutions::Table, CommandExecutions::CommandId)
                            .to(Commands::Table, Commands::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommandExecutions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CommandExecutions {
    Table,
    Id,
    CommandId,
    Metadata,
    CreatedAt,
}
