//! # Create Items Impact Sounds Junction Table
//!
//! Migration that creates the "items_impact_sounds" junction table which stores
//! the connection between items and any number of sounds

use sea_orm_migration::{prelude::*, schema::*};

use super::{
    m20241208_060123_create_items_table::Items, m20241208_060144_create_sounds_table::Sounds,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ItemsSounds::Table)
                    .if_not_exists()
                    .col(uuid(ItemsSounds::ItemId))
                    .col(uuid(ItemsSounds::SoundId))
                    .col(string(ItemsSounds::SoundType))
                    // Junction table uses a composite key of the item, sound id and sound type combined
                    .primary_key(
                        Index::create()
                            .name("pk_items_sounds")
                            .col(ItemsSounds::ItemId)
                            .col(ItemsSounds::SoundId)
                            .col(ItemsSounds::SoundType),
                    )
                    // Connect to items table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_items_sounds_item_id")
                            .from(ItemsSounds::Table, ItemsSounds::ItemId)
                            .to(Items::Table, Items::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // Connect to sounds table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_items_sounds_sound_id")
                            .from(ItemsSounds::Table, ItemsSounds::SoundId)
                            .to(Sounds::Table, Sounds::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ItemsSounds::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ItemsSounds {
    Table,
    ItemId,
    SoundId,
    SoundType,
}
