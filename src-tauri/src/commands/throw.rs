use tokio::sync::broadcast;

use crate::{http::server::EventMessage, state::app_data::ThrowableConfig};

#[tauri::command]
pub fn test_throw(
    config: ThrowableConfig,
    amount: Option<u32>,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> Result<bool, ()> {
    if let Some(amount) = amount {
        event_sender
            .send(EventMessage::ThrowMany { config, amount })
            .map_err(|_| ())?;
    } else {
        event_sender
            .send(EventMessage::Throw { config })
            .map_err(|_| ())?;
    }

    Ok(true)
}
