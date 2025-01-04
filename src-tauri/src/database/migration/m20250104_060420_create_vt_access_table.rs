use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VTAccess::Table)
                    .if_not_exists()
                    .col(pk_auto(VTAccess::Id))
                    .col(string(VTAccess::AccessToken))
                    .col(date_time(VTAccess::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VTAccess::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum VTAccess {
    Table,
    Id,
    AccessToken,
    CreatedAt,
}
