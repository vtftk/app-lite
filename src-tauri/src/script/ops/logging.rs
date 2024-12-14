//! # Logging (JS API)
//!
//! API for performing logging from the JS runtime

use deno_core::*;

use crate::script::runtime::RuntimeExecutionContext;

fn exec_prefix(ctx: Option<RuntimeExecutionContext>) -> String {
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
    let prefix = exec_prefix(ctx);
    log::debug!("{prefix}: {}", message);
}

#[op2]
pub fn op_log_info(#[serde] ctx: Option<RuntimeExecutionContext>, #[string] message: String) {
    let prefix = exec_prefix(ctx);
    log::info!("{prefix}: {}", message);
}

#[op2]
pub fn op_log_warn(#[serde] ctx: Option<RuntimeExecutionContext>, #[string] message: String) {
    let prefix = exec_prefix(ctx);
    log::warn!("{prefix}: {}", message);
}

#[op2]
pub fn op_log_error(#[serde] ctx: Option<RuntimeExecutionContext>, #[string] message: String) {
    let prefix = exec_prefix(ctx);
    log::error!("{prefix}: {}", message);
}
