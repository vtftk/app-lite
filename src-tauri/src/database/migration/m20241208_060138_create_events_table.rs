use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .if_not_exists()
                    .col(pk_uuid(Events::Id))
                    .col(boolean(Events::Enabled))
                    .col(string(Events::Name))
                    .col(json(Events::Trigger))
                    .col(json(Events::Outcome))
                    .col(integer(Events::Cooldown))
                    .col(string(Events::RequireRole))
                    .col(integer(Events::OutcomeDelay))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
    Enabled,
    Name,
    Trigger,
    Outcome,
    Cooldown,
    RequireRole,
    OutcomeDelay,
}
