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
                    .table(CommandAlias::Table)
                    .if_not_exists()
                    .col(pk_uuid(CommandAlias::Id))
                    .col(uuid(CommandAlias::CommandId))
                    .col(string(CommandAlias::Alias))
                    .col(integer(CommandAlias::Order))
                    // Connect to commands table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_command_aliases_command_id")
                            .from(CommandAlias::Table, CommandAlias::CommandId)
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
            .drop_table(Table::drop().table(CommandAlias::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CommandAlias {
    Table,
    Id,
    CommandId,
    Alias,
    Order,
}
