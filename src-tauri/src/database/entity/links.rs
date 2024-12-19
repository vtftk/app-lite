use sea_orm::{Linked, RelationTrait};

use crate::database::entity::{item_collection_items, items_impact_sounds};

/// Relationship linking item to its impact sounds using the
/// junction table
pub struct ItemImpactSounds;

impl Linked for ItemImpactSounds {
    type FromEntity = super::items::Entity;
    type ToEntity = super::sounds::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            items_impact_sounds::Relation::Item.def().rev(),
            items_impact_sounds::Relation::Sound.def(),
        ]
    }
}
/// Relationship linking item to its impact sounds using the
/// junction table
pub struct ItemCollectionItems;

impl Linked for ItemCollectionItems {
    type FromEntity = super::item_collections::Entity;
    type ToEntity = super::items::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            item_collection_items::Relation::Item.def().rev(),
            item_collection_items::Relation::Collection.def(),
        ]
    }
}
