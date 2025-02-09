use sea_orm::{Linked, RelationTrait};

use crate::database::entity::items_sounds;

/// Relationship linking item to its impact sounds using the
/// junction table
pub struct ItemSounds;

impl Linked for ItemSounds {
    type FromEntity = super::items::Entity;
    type ToEntity = super::sounds::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            items_sounds::Relation::Item.def().rev(),
            items_sounds::Relation::Sound.def(),
        ]
    }
}
