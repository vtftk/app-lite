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
                    .table(ScriptLogs::Table)
                    .if_not_exists()
                    .col(pk_uuid(ScriptLogs::Id))
                    .col(uuid(ScriptLogs::ScriptId))
                    .col(integer(ScriptLogs::Level))
                    .col(string(ScriptLogs::Message))
                    .col(date_time(ScriptLogs::CreatedAt))
                    // Connect to scripts table
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_script_logs_script_id")
                            .from(ScriptLogs::Table, ScriptLogs::ScriptId)
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
            .drop_table(Table::drop().table(ScriptLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ScriptLogs {
    Table,
    Id,
    ScriptId,
    Level,
    Message,
    CreatedAt,
}
