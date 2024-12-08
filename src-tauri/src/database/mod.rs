use std::path::Path;

use log::warn;
use migration::Migrator;
use sea_orm::sqlx::ConnectOptions;
use sea_orm::DatabaseConnection;
use sea_orm::{
    sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Database,
};
use sea_orm_migration::MigratorTrait;

pub mod entity;
mod migration;

/// Connects to the SQLite database at the provided path, creating a
/// new database file if none exist
pub async fn connect_database(path: &Path) -> anyhow::Result<DatabaseConnection> {
    // Create connection URL
    let url = SqliteConnectOptions::new()
        .journal_mode(SqliteJournalMode::Wal)
        .filename(path)
        .optimize_on_close(true, None)
        .create_if_missing(true)
        .to_url_lossy();

    let options = sea_orm::ConnectOptions::new(url);
    let db = Database::connect(options).await?;

    if let Err(err) = Migrator::up(&db, None).await {
        warn!("failed to apply/check database migrations: {:?}", err);
        // TODO: Check for applied forward migrations, these are not always failing changes
    }

    Ok(db)
}
