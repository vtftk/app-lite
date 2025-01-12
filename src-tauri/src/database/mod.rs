use anyhow::Context;
use chrono::{Days, Utc};
use entity::{
    app_data::AppDataModel, command_executions::CommandExecutionModel,
    command_logs::CommandLogsModel, event_executions::EventExecutionModel,
    event_logs::EventLogsModel,
};
use log::warn;
use migration::Migrator;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, File};

pub mod entity;
mod migration;

/// Connects to the SQLite database at the provided path, creating a
/// new database file if none exist
pub async fn connect_database(path: PathBuf) -> anyhow::Result<DatabaseConnection> {
    if !path.exists() {
        let parent = path.parent().context("database path invalid")?;
        create_dir_all(parent)
            .await
            .context("create database path")?;

        File::create(&path).await?;
    }

    let path = path.to_str().context("invalid db path")?;

    let path = format!("sqlite://{path}");

    let options = sea_orm::ConnectOptions::new(path);
    let db = Database::connect(options).await?;

    setup_database(&db).await?;

    Ok(db)
}

pub async fn setup_database(db: &DatabaseConnection) -> anyhow::Result<()> {
    if let Err(err) = Migrator::up(db, None).await {
        warn!("failed to apply/check database migrations: {:?}", err);
        // TODO: Check for applied forward migrations, these are not always failing changes
    }

    Ok(())
}

pub async fn clean_old_data(db: DatabaseConnection) -> anyhow::Result<()> {
    let main_config = AppDataModel::get_main_config(&db).await?;

    let now = Utc::now();

    // Clean logs
    if main_config.clean_logs {
        let clean_logs_date = now
            .checked_sub_days(Days::new(main_config.clean_logs_days))
            .context("system time is incorrect")?;

        EventLogsModel::delete_before(&db, clean_logs_date).await?;
        CommandLogsModel::delete_before(&db, clean_logs_date).await?;
    }

    // Clean executions
    if main_config.clean_executions {
        let clean_executions_date = now
            .checked_sub_days(Days::new(main_config.clean_executions_days))
            .context("system time is incorrect")?;

        CommandExecutionModel::delete_before(&db, clean_executions_date).await?;
        EventExecutionModel::delete_before(&db, clean_executions_date).await?;
    }

    Ok(())
}
