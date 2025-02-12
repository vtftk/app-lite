use sea_orm::prelude::*;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

pub type DbResult<T> = Result<T, DbErr>;

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MinimumRequireRole {
    #[sea_orm(string_value = "None")]
    #[default]
    None,
    #[sea_orm(string_value = "Follower")]
    Follower,
    #[sea_orm(string_value = "Vip")]
    Vip,
    #[sea_orm(string_value = "Mod")]
    Mod,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionsQuery {
    pub start_date: Option<DateTimeUtc>,
    pub end_date: Option<DateTimeUtc>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Default, Deserialize)]
pub struct UpdateOrdering {
    pub id: Uuid,
    pub order: u32,
}
