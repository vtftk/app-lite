use std::sync::Arc;

use log::error;
use tauri::{AppHandle, Emitter};
use tokio::{
    sync::{broadcast, Mutex},
    task::AbortHandle,
};
use twitch_api::{
    eventsub::{
        self,
        channel::{
            channel_points_custom_reward_redemption::Reward, chat::message::Cheer,
            subscription::message::SubscriptionMessage,
        },
    },
    twitch_oauth2::UserToken,
    types::{DisplayName, RedemptionId, SubscriptionTier, UserId, UserName},
    HelixClient,
};

use super::websocket::WebsocketClient;

pub struct TwitchManager {
    pub helix_client: HelixClient<'static, reqwest::Client>,
    state: Mutex<TwitchManagerState>,
    tx: broadcast::Sender<TwitchEvent>,
    app_handle: AppHandle,
}

impl TwitchManager {
    pub fn new(
        helix_client: HelixClient<'static, reqwest::Client>,
        app_handle: AppHandle,
    ) -> (Arc<Self>, broadcast::Receiver<TwitchEvent>) {
        let (tx, rx) = broadcast::channel(10);
        (
            Arc::new(Self {
                helix_client,
                state: Default::default(),
                tx,
                app_handle,
            }),
            rx,
        )
    }

    pub async fn is_authenticated(&self) -> bool {
        let lock = &*self.state.lock().await;
        matches!(lock, TwitchManagerState::Authenticated { .. })
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum TwitchEvent {
    Redeem {
        id: RedemptionId,
        reward: Reward,
        user_id: UserId,
        user_name: UserName,
        user_display_name: DisplayName,
    },
    CheerBits {
        // Total bits gifted
        bits: i64,
        anonymous: bool,

        // User details empty when cheer is anonymous
        user_id: Option<UserId>,
        user_name: Option<UserName>,
        user_display_name: Option<DisplayName>,
    },
    Follow {
        user_id: UserId,
        user_name: UserName,
        user_display_name: DisplayName,
    },
    Sub {
        is_gift: bool,
        tier: SubscriptionTier,
        user_id: UserId,
        user_name: UserName,
        user_display_name: DisplayName,
    },
    GiftSub {
        anonymous: bool,

        // Total subs gifted
        total: i64,

        // Total gifts user has given (If not anonymous)
        cumulative_total: Option<i64>,
        tier: SubscriptionTier,

        // User details empty when cheer is anonymous
        user_id: Option<UserId>,
        user_name: Option<DisplayName>,
        user_display_name: Option<UserName>,
    },
    ResubMsg {
        cumulative_months: i64,
        duration_months: i64,
        message: SubscriptionMessage,
        streak_months: Option<i64>,
        tier: SubscriptionTier,
        user_id: UserId,
        user_name: UserName,
        user_display_name: DisplayName,
    },
    ChatMsg {
        user_id: UserId,
        user_name: UserName,
        user_display_name: UserName,
        message: eventsub::channel::chat::Message,
        cheer: Option<Cheer>,
    },
}

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
        // Currently active websocket connection
        _websocket: WebsocketManagedTask,
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
        twitch_manager: Arc<TwitchManager>,
        tx: broadcast::Sender<TwitchEvent>,
        token: UserToken,
    ) -> WebsocketManagedTask {
        let abort_handle = tokio::spawn(async move {
            let ws = WebsocketClient::new(twitch_manager.helix_client.clone(), tx, token);
            if let Err(err) = ws.run().await {
                error!("websocket error: {:?}", err);
                twitch_manager.reset().await;
            }
        })
        .abort_handle();

        WebsocketManagedTask(abort_handle)
    }
}

impl TwitchManager {
    pub async fn set_authenticated(self: &Arc<Self>, token: UserToken) {
        {
            let lock = &mut *self.state.lock().await;

            let websocket =
                WebsocketManagedTask::create(self.clone(), self.tx.clone(), token.clone());

            *lock = TwitchManagerState::Authenticated {
                token,
                _websocket: websocket,
            };
        }

        // Tell the app we are authenticated
        _ = self.app_handle.emit("authenticated", ());
    }

    pub async fn reset(&self) {
        {
            let lock = &mut *self.state.lock().await;
            *lock = TwitchManagerState::Initial;
        }

        // Tell the app we are authenticated
        _ = self.app_handle.emit("logout", ());
    }

    pub fn events(&self) -> broadcast::Receiver<TwitchEvent> {
        self.tx.subscribe()
    }
}
