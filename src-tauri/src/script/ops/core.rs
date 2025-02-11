use deno_core::op2;
use uuid::Uuid;

/// Generates a random UUID returning it in string form for the JS
/// scripting engine
#[op2]
#[string]
pub fn op_uuid_v4() -> String {
    Uuid::new_v4().to_string()
}
