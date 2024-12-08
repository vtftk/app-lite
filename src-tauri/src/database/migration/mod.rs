pub use sea_orm_migration::prelude::*;

mod m20241208_060123_create_items_table;
mod m20241208_060138_create_events_table;
mod m20241208_060144_create_sounds_table;
mod m20241208_060150_create_scripts_table;
mod m20241208_060200_create_commands_table;
mod m20241208_060230_create_model_data_table;
mod m20241208_063859_create_items_impact_sounds_junction_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241208_060123_create_items_table::Migration),
            Box::new(m20241208_060144_create_sounds_table::Migration),
            Box::new(m20241208_063859_create_items_impact_sounds_junction_table::Migration),
            Box::new(m20241208_060138_create_events_table::Migration),
            Box::new(m20241208_060150_create_scripts_table::Migration),
            Box::new(m20241208_060200_create_commands_table::Migration),
            Box::new(m20241208_060230_create_model_data_table::Migration),
        ]
    }
}
