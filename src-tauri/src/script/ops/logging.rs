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
            RuntimeExecutionContext::Event { event_id } => format!("[event:{event_id}]"),
            RuntimeExecutionContext::Command { command_id } => format!("[command:{command_id}]"),
        },
        None => "[unknown]".to_string(),
    }
}

#[op2]
pub fn op_log(
    #[serde] ctx: Option<RuntimeExecutionContext>,
    #[serde] level: LoggingLevelDb,
    #[string] message: String,
) {
    let prefix = exec_prefix(ctx.as_ref());

    let log_level = match &level {
        LoggingLevelDb::Debug => log::Level::Debug,
        LoggingLevelDb::Info => log::Level::Info,
        LoggingLevelDb::Warn => log::Level::Warn,
        LoggingLevelDb::Error => log::Level::Error,
    };

    log::log!(log_level, "{prefix}: {message}");

    if let Some(ctx) = ctx {
        tokio::spawn(global_script_event(LogPersistEvent {
            ctx,
            level,
            message,
            created_at: Utc::now(),
        }));
    }
}
