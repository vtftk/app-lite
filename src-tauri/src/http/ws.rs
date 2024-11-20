use axum::extract::ws::{self, WebSocket};
use serde::{Deserialize, Serialize};
use tokio::{select, sync::broadcast};
use uuid::Uuid;

pub async fn handle_socket(socket: WebSocket, mut recv_handle: EventRecvHandle) {
    let mut socket_state = WebSocketSession {
        socket,
        session_id: Uuid::new_v4(),
    };

    loop {
        select! {
            // Handle messages from the web socket connection
            msg = socket_state.socket.recv() => {

                let msg = match msg {
                    Some(Ok(msg))=> msg,
                    _ => return,
                };

                if let Err(_err) = handle_socket_msg(&mut socket_state, msg).await {
                    return;
                }
            },

            // Handle messages to broadcast to the web socket
            broadcast_msg = recv_handle.0.recv() => {
                if let Ok(broadcast_msg) = broadcast_msg {
                    if let Err(_err) = handle_socket_broadcast(&mut socket_state, broadcast_msg).await {
                        return;
                    }
                }
            }
        }
    }
}

/// Session data for a web socket
pub struct WebSocketSession {
    socket: WebSocket,
    /// Unique ID for the session
    session_id: Uuid,
}

async fn handle_socket_msg(
    _socket_state: &mut WebSocketSession,
    msg: axum::extract::ws::Message,
) -> anyhow::Result<()> {
    // Parse the incoming message
    let msg: WebsocketClientMessage = match msg {
        axum::extract::ws::Message::Text(value) => serde_json::from_str(&value)?,
        axum::extract::ws::Message::Binary(value) => serde_json::from_slice(&value)?,
        _ => return Ok(()),
    };

    // let msg = serde_json::to_string(&WebsocketServerMessage::Authenticated)?;
    // socket_state.socket.send(ws::Message::Text(msg)).await?;

    Ok(())
}

async fn handle_socket_broadcast(
    socket_state: &mut WebSocketSession,
    msg: (WebsocketServerMessage, Option<Uuid>),
) -> anyhow::Result<()> {
    let (msg, session_id) = msg;

    // Ensure targeted message is actually for us
    if let Some(session_id) = session_id {
        if session_id != socket_state.session_id {
            return Ok(());
        }
    }

    let msg = serde_json::to_string(&msg)?;
    socket_state.socket.send(ws::Message::Text(msg)).await?;

    Ok(())
}

/// Messages sent by the websocket server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketServerMessage {
    AuthStateChange { state: AuthStateChange },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthStateChange {
    Authenticated,
    NotAuthenticated,
}

/// Messages received from the websocket client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketClientMessage {}

pub fn create_event_handles() -> (EventSendHandle, EventRecvHandle) {
    let (tx, rx) = broadcast::channel(10);

    (EventSendHandle(tx), EventRecvHandle(rx))
}

/// Handle to broadcast messages to websocket clients
#[derive(Clone)]
pub struct EventSendHandle(pub broadcast::Sender<(WebsocketServerMessage, Option<Uuid>)>);

impl EventSendHandle {
    pub fn send(&self, msg: WebsocketServerMessage) {
        _ = self.0.send((msg, None))
    }
}

/// Handle to receive messages for websocket clients
pub struct EventRecvHandle(broadcast::Receiver<(WebsocketServerMessage, Option<Uuid>)>);

impl Clone for EventRecvHandle {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}
