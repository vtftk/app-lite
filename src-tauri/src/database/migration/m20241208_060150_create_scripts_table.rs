use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Scripts::Table)
                    .if_not_exists()
                    .col(pk_uuid(Scripts::Id))
                    .col(boolean(Scripts::Enabled))
                    .col(string(Scripts::Name))
                    .col(text(Scripts::Script))
                    .col(integer(Scripts::Order))
                    .col(date_time(Scripts::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Scripts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Scripts {
    Table,
    Id,
    Enabled,
    Name,
    Script,
    Order,
    CreatedAt,
}
