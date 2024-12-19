use sea_orm_migration::{prelude::*, schema::*};

use super::{
    m20241208_060123_create_items_table::Items,
    m20241219_090305_create_item_collections_table::ItemCollections,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ItemCollectionItems::Table)
                    .if_not_exists()
                    .col(uuid(ItemCollectionItems::ItemId))
                    .col(uuid(ItemCollectionItems::ItemCollectionId))
                    .col(integer(ItemCollectionItems::Order))
                    // Junction table uses a composite key of the item and sound ids combined
                    .primary_key(
                        Index::create()
                            .name("pk_item_collection_items")
                            .col(ItemCollectionItems::ItemId)
                            .col(ItemCollectionItems::ItemCollectionId),
                    )
                    // Connect to items table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_items_collection_items_item_id")
                            .from(ItemCollectionItems::Table, ItemCollectionItems::ItemId)
                            .to(Items::Table, Items::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // Connect to collections table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_items_collection_items_collection_id")
                            .from(
                                ItemCollectionItems::Table,
                                ItemCollectionItems::ItemCollectionId,
                            )
                            .to(ItemCollections::Table, ItemCollections::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ItemCollectionItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ItemCollectionItems {
    Table,
    ItemId,
    ItemCollectionId,
    Order,
}
