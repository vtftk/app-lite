use sea_orm::prelude::*;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

pub type DbResult<T> = Result<T, DbErr>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MinimumRequireRole {
    #[sea_orm(string_value = "None")]
    None,
    #[sea_orm(string_value = "Mod")]
    Mod,
    #[sea_orm(string_value = "Vip")]
    Vip,
    #[sea_orm(string_value = "Broadcaster")]
    Broadcaster,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinMax<T> {
    /// Minimum value
    pub min: T,
    /// Maximum value
    pub max: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum LoggingLevelDb {
    #[sea_orm(num_value = 0)]
    Debug,
    #[sea_orm(num_value = 1)]
    Info,
    #[sea_orm(num_value = 2)]
    Warn,
    #[sea_orm(num_value = 3)]
    Error,
}
