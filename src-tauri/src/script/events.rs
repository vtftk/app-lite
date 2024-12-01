use std::sync::Arc;

use anyhow::Context;
use serde::Serialize;
use tokio::sync::{mpsc, oneshot, Mutex};
use twitch_api::helix::chat::{
    SendChatMessageBody, SendChatMessageRequest, SendChatMessageResponse,
};

use crate::{state::app_data::AppDataStore, twitch::manager::TwitchManager};

/// Events coming from the JS runtime to be executed by a locally
/// managed handler with state accessible only from the main app
pub enum JsEventMessage {
    /// Trigger sending a twitch chat message
    TwitchSendChat {
        /// The message to send
        message: String,
        /// Channel to respond through with the outcome to sending the message
        return_tx: oneshot::Sender<anyhow::Result<()>>,
    },
}

/// Event coming from outside the JS runtime to trigger executing
/// code within the runtime event listeners
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ScriptExecuteEvent {
    Chat { message: String },
}

/// Currently active sender for [JsEventMessage]s
pub static SCRIPT_EVENT_PRODUCER: Mutex<Option<mpsc::Sender<JsEventMessage>>> =
    Mutex::const_new(None);

/// Initializes global script handling using the provided dependencies
pub fn init_script_event_handling(
    app_data_store: AppDataStore,
    twitch_manager: Arc<TwitchManager>,
) {
    let (tx, rx) = mpsc::channel(10);

    // Can block here, initialization will never have any other writers so won't be blocked
    *SCRIPT_EVENT_PRODUCER.blocking_lock() = Some(tx);

    // Spawn background task to process events
    tauri::async_runtime::spawn(handle_script_events(app_data_store, twitch_manager, rx));
}

/// Asynchronous task handling for receiving events then dispatching tasks
/// to executed the event action
pub async fn handle_script_events(
    _app_data_store: AppDataStore,
    twitch_manager: Arc<TwitchManager>,
    mut rx: mpsc::Receiver<JsEventMessage>,
) {
    while let Some(event) = rx.recv().await {
        match event {
            JsEventMessage::TwitchSendChat { message, return_tx } => {
                let twitch_manager = twitch_manager.clone();
                tokio::spawn(async move {
                    let result = handle_twitch_send_chat_event(twitch_manager, message).await;
                    _ = return_tx.send(result);
                });
            }
        }
    }
}

/// Handles a twitch chat event, sends a twitch chat message
async fn handle_twitch_send_chat_event(
    twitch_manager: Arc<TwitchManager>,
    message: String,
) -> anyhow::Result<()> {
    let token = twitch_manager
        .get_user_token()
        .await
        .context("not authenticated")?;
    let user_id = token.user_id.clone();
    let request = SendChatMessageRequest::new();
    let body = SendChatMessageBody::new(user_id.clone(), user_id, message);
    let _response: SendChatMessageResponse = twitch_manager
        .helix_client
        .req_post(request, body, &token)
        .await?
        .data;

    Ok(())
}
