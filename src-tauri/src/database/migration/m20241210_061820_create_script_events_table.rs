//! # Script Events Table
//!
//! One to many relationship from scripts to any number
//! of events that a script may be subscribed to

use sea_orm_migration::{prelude::*, schema::*};

use super::m20241208_060150_create_scripts_table::Scripts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ScriptEvents::Table)
                    .if_not_exists()
                    .col(uuid(ScriptEvents::ScriptId))
                    .col(string(ScriptEvents::Event))
                    // Composite primary key from the script and event
                    .primary_key(
                        Index::create()
                            .name("pk_script_events")
                            .col(ScriptEvents::ScriptId)
                            .col(ScriptEvents::Event),
                    )
                    // Connect to items table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_script_events_script_id")
                            .from(ScriptEvents::Table, ScriptEvents::ScriptId)
                            .to(Scripts::Table, Scripts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ScriptEvents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ScriptEvents {
    Table,
    ScriptId,
    Event,
}
