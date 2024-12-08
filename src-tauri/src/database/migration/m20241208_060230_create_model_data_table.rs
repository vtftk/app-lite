use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ModelData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelData::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(json(ModelData::Calibration))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ModelData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ModelData {
    Table,
    Id,
    Calibration,
}
