use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Commands::Table)
                    .if_not_exists()
                    .col(pk_uuid(Commands::Id))
                    .col(boolean(Commands::Enabled))
                    .col(string(Commands::Name))
                    .col(text(Commands::Command))
                    .col(json(Commands::Aliases))
                    .col(json(Commands::Outcome))
                    .col(integer(Commands::Cooldown))
                    .col(string(Commands::RequireRole))
                    .col(integer(Commands::Order))
                    .col(date_time(Commands::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Commands::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Commands {
    Table,
    Id,
    Enabled,
    Name,
    Command,
    Aliases,
    Outcome,
    Cooldown,
    RequireRole,
    Order,
    CreatedAt,
}
