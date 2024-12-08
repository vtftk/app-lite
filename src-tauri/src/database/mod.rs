use std::path::Path;

use sea_orm::sqlx::ConnectOptions;
use sea_orm::DatabaseConnection;
use sea_orm::{
    sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Database,
};

mod entity;
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
    Ok(db)
}
