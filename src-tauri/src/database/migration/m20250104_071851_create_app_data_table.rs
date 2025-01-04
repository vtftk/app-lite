use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AppData::Table)
                    .if_not_exists()
                    .col(pk_auto(AppData::Id))
                    .col(json_binary(AppData::Data))
                    .col(date_time(AppData::CreatedAt))
                    .col(date_time(AppData::LastModifiedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AppData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AppData {
    Table,
    Id,
    Data,
    CreatedAt,
    LastModifiedAt,
}
