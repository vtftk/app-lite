//! # Logging (JS API)
//!
//! API for performing logging from the JS runtime

use chrono::Utc;
use deno_core::*;

use crate::{
    database::entity::shared::LoggingLevelDb,
    script::{
        events::{global_script_event, LogPersistEvent},
        runtime::RuntimeExecutionContext,
    },
};

fn exec_prefix(ctx: Option<&RuntimeExecutionContext>) -> String {
    match ctx {
        Some(ctx) => match ctx {
            RuntimeExecutionContext::Script { script_id } => format!("[script:{script_id}]"),
            RuntimeExecutionContext::Command { command_id } => format!("[command:{command_id}]"),
        },
        None => "[unknown]".to_string(),
    }
}

#[op2]
pub fn op_log_debug(#[serde] ctx: Option<RuntimeExecutionContext>, #[string] message: String) {
    let prefix = exec_prefix(ctx.as_ref());
    log::debug!("{prefix}: {message}");

    if let Some(ctx) = ctx {
        tokio::spawn(global_script_event(LogPersistEvent {
            ctx,
            level: LoggingLevelDb::Debug,
            message,
            created_at: Utc::now(),
        }));
    }
}

#[op2]
pub fn op_log_info(#[serde] ctx: Option<RuntimeExecutionContext>, #[string] message: String) {
    let prefix = exec_prefix(ctx.as_ref());
    log::info!("{prefix}: {message}");

    if let Some(ctx) = ctx {
        tokio::spawn(global_script_event(LogPersistEvent {
            ctx,
            level: LoggingLevelDb::Info,
            message,
            created_at: Utc::now(),
        }));
    }
}

#[op2]
pub fn op_log_warn(#[serde] ctx: Option<RuntimeExecutionContext>, #[string] message: String) {
    let prefix = exec_prefix(ctx.as_ref());
    log::warn!("{prefix}: {message}");

    if let Some(ctx) = ctx {
        tokio::spawn(global_script_event(LogPersistEvent {
            ctx,
            level: LoggingLevelDb::Warn,
            message,
            created_at: Utc::now(),
        }));
    }
}

#[op2]
pub fn op_log_error(#[serde] ctx: Option<RuntimeExecutionContext>, #[string] message: String) {
    let prefix = exec_prefix(ctx.as_ref());
    log::error!("{prefix}: {message}");

    if let Some(ctx) = ctx {
        tokio::spawn(global_script_event(LogPersistEvent {
            ctx,
            level: LoggingLevelDb::Error,
            message,
            created_at: Utc::now(),
        }));
    }
}
