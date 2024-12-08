//! # Create Items Impact Sounds Junction Table
//!
//! Migration that creates the "items_impact_sounds" junction table which stores
//! the connection between items and any number of sounds

use sea_orm_migration::{prelude::*, schema::uuid};

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
                    .table(ItemsImpactSounds::Table)
                    .if_not_exists()
                    .col(uuid(ItemsImpactSounds::ItemId))
                    .col(uuid(ItemsImpactSounds::SoundId))
                    // Junction table uses a composite key of the item and sound ids combined
                    .primary_key(
                        Index::create()
                            .name("pk_items_impact_sounds")
                            .col(ItemsImpactSounds::ItemId)
                            .col(ItemsImpactSounds::SoundId),
                    )
                    // Connect to items table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_items_impact_sounds_item_id")
                            .from(ItemsImpactSounds::Table, ItemsImpactSounds::ItemId)
                            .to(Items::Table, Items::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // Connect to sounds table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_items_impact_sounds_sound_id")
                            .from(ItemsImpactSounds::Table, ItemsImpactSounds::SoundId)
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
            .drop_table(Table::drop().table(ItemsImpactSounds::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ItemsImpactSounds {
    Table,
    ItemId,
    SoundId,
}
