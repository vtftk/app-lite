use serde::Serialize;

pub mod calibration;
#[allow(clippy::module_inception)]
pub mod commands;
pub mod data;
pub mod events;
pub mod items;
pub mod scripts;
pub mod sounds;
pub mod test;
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

impl<E> From<E> for CmdError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        CmdError(value.into())
    }
}
