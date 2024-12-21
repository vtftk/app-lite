//! # Create Items Table
//!
//! Migration that creates the "items" table which stores
//! throwable items

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .if_not_exists()
                    .col(pk_uuid(Items::Id))
                    .col(string(Items::Name))
                    .col(json(Items::Image))
                    .col(integer(Items::Order))
                    .col(date_time(Items::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Items {
    Table,
    Id,
    Name,
    Image,
    Order,
    CreatedAt,
}
