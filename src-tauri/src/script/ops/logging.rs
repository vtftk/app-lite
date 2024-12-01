//! # Logging (JS API)
//!
//! API for performing logging from the JS runtime

use deno_core::*;

#[op2(fast)]
pub fn op_log_debug(#[string] message: String) {
    log::debug!("[script]: {}", message);
}

#[op2(fast)]
pub fn op_log_info(#[string] message: String) {
    log::info!("[script]: {}", message);
}

#[op2(fast)]
pub fn op_log_warn(#[string] message: String) {
    log::warn!("[script]: {}", message);
}

#[op2(fast)]
pub fn op_log_error(#[string] message: String) {
    log::error!("[script]: {}", message);
}
