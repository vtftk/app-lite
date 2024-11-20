use log::error;
use tokio::{
    sync::{broadcast, mpsc, Mutex},
    task::AbortHandle,
};
use twitch_api::{twitch_oauth2::UserToken, types::UserId, HelixClient};

use super::websocket::WebsocketClient;

pub struct TwitchManager {
    helix_client: HelixClient<'static, reqwest::Client>,
    state: Mutex<TwitchManagerState>,
    tx: broadcast::Sender<TwitchEvent>,
}

impl TwitchManager {
    pub fn new(
        helix_client: HelixClient<'static, reqwest::Client>,
    ) -> (Self, broadcast::Receiver<TwitchEvent>) {
        let (tx, rx) = broadcast::channel(10);
        (
            Self {
                helix_client,
                state: Default::default(),
                tx,
            },
            rx,
        )
    }

    pub async fn is_authenticated(&self) -> bool {
        let lock = &*self.state.lock().await;
        matches!(lock, TwitchManagerState::Authenticated { .. })
    }
}

#[derive(Debug, Clone)]
pub enum TwitchEvent {}

#[derive(Default)]
#[allow(clippy::large_enum_variant)]
enum TwitchManagerState {
    // Twitch is not yet authenticated
    #[default]
    Initial,
    // Twitch is authenticated
    Authenticated {
        // Token for the authenticated user
        token: UserToken,
        // User ID of the user to monitor
        user_id: UserId,
        // Currently active websocket connection
        websocket: WebsocketManagedTask,
    },
}

struct WebsocketManagedTask(AbortHandle);

impl Drop for WebsocketManagedTask {
    fn drop(&mut self) {
        self.0.abort();
    }
}

impl WebsocketManagedTask {
    pub fn create(
        client: HelixClient<'static, reqwest::Client>,
        token: UserToken,
    ) -> WebsocketManagedTask {
        let abort_handle = tokio::spawn(async move {
            let ws = WebsocketClient::new(None, token, client);
            if let Err(err) = ws.run().await {
                error!("failed to connect: {:?}", err);
            }
        })
        .abort_handle();

        WebsocketManagedTask(abort_handle)
    }
}

impl TwitchManager {
    pub async fn set_authenticated(&self, token: UserToken) -> anyhow::Result<()> {
        let lock = &mut *self.state.lock().await;
        let user_id = token.user_id.clone();

        let websocket = WebsocketManagedTask::create(self.helix_client.clone(), token.clone());

        *lock = TwitchManagerState::Authenticated {
            user_id,
            token,
            websocket,
        };

        Ok(())
    }

    pub async fn reset(&self) {
        let lock = &mut *self.state.lock().await;
        *lock = TwitchManagerState::Initial;
    }

    pub fn events(&self) -> broadcast::Receiver<TwitchEvent> {
        self.tx.subscribe()
    }
}
