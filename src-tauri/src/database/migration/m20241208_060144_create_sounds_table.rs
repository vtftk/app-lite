use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sounds::Table)
                    .if_not_exists()
                    .col(pk_uuid(Sounds::Id))
                    .col(string(Sounds::Name))
                    .col(string(Sounds::Src))
                    .col(float(Sounds::Volume))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sounds::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sounds {
    Table,
    Id,
    Name,
    Src,
    Volume,
}
