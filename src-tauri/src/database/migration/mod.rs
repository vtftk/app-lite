pub use sea_orm_migration::prelude::*;

mod m20241208_060123_create_items_table;
mod m20241208_060138_create_events_table;
mod m20241208_060144_create_sounds_table;
mod m20241208_060200_create_commands_table;
mod m20241208_060230_create_model_data_table;
mod m20241208_063859_create_items_impact_sounds_junction_table;
mod m20241210_082256_create_event_executions_table;
mod m20241210_082316_create_command_executions_table;
mod m20241211_102725_seed_defaults;
mod m20241212_114700_create_key_value_table;
mod m20241214_080902_create_command_logs_table;
mod m20241227_110419_create_event_logs_table;
mod m20250104_053253_create_twitch_access_table;
mod m20250104_060420_create_vt_access_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241208_060123_create_items_table::Migration),
            Box::new(m20241208_060138_create_events_table::Migration),
            Box::new(m20241208_060144_create_sounds_table::Migration),
            Box::new(m20241208_060200_create_commands_table::Migration),
            Box::new(m20241208_060230_create_model_data_table::Migration),
            Box::new(m20241208_063859_create_items_impact_sounds_junction_table::Migration),
            Box::new(m20241210_082256_create_event_executions_table::Migration),
            Box::new(m20241210_082316_create_command_executions_table::Migration),
            Box::new(m20241211_102725_seed_defaults::Migration),
            Box::new(m20241212_114700_create_key_value_table::Migration),
            Box::new(m20241214_080902_create_command_logs_table::Migration),
            Box::new(m20241227_110419_create_event_logs_table::Migration),
            Box::new(m20250104_053253_create_twitch_access_table::Migration),
            Box::new(m20250104_060420_create_vt_access_table::Migration),
        ]
    }
}
