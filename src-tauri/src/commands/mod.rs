use serde::Serialize;

pub mod auth;
pub mod calibration;
pub mod data;
pub mod edit;
pub mod throw;
pub mod twitch;

type CmdResult<T> = Result<T, CmdError>;

pub struct CmdError(anyhow::Error);

impl Serialize for CmdError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self.0))
    }
}

impl From<anyhow::Error> for CmdError {
    fn from(value: anyhow::Error) -> Self {
        CmdError(value)
    }
}
