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
                    .table(CommandLogs::Table)
                    .if_not_exists()
                    .col(pk_uuid(CommandLogs::Id))
                    .col(uuid(CommandLogs::CommandId))
                    .col(integer(CommandLogs::Level))
                    .col(string(CommandLogs::Message))
                    .col(date_time(CommandLogs::CreatedAt))
                    // Connect to commands table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_command_logs_command_id")
                            .from(CommandLogs::Table, CommandLogs::CommandId)
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
            .drop_table(Table::drop().table(CommandLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CommandLogs {
    Table,
    Id,
    CommandId,
    Level,
    Message,
    CreatedAt,
}
