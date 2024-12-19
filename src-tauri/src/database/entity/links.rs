use sea_orm::{Linked, RelationTrait};

use crate::database::entity::items_impact_sounds;

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
