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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinMax<T> {
    /// Minimum value
    pub min: T,
    /// Maximum value
    pub max: T,
}
