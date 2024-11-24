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

    pub async fn get_user_token(&self) -> Option<UserToken> {
        let lock = &*self.state.lock().await;
        match lock {
            TwitchManagerState::Initial => None,
            TwitchManagerState::Authenticated { token, _websocket } => Some(token.clone()),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TwitchEventRedeem {
    pub id: RedemptionId,
    pub reward: Reward,
    pub user_id: UserId,
    pub user_name: UserName,
    pub user_display_name: DisplayName,
}
#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TwitchEventCheerBits {
    // Total bits gifted
    pub bits: i64,
    pub anonymous: bool,

    // User details empty when cheer is anonymous
    pub user_id: Option<UserId>,
    pub user_name: Option<UserName>,
    pub user_display_name: Option<DisplayName>,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TwitchEventFollow {
    pub user_id: UserId,
    pub user_name: UserName,
    pub user_display_name: DisplayName,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TwitchEventSub {
    pub is_gift: bool,
    pub tier: SubscriptionTier,
    pub user_id: UserId,
    pub user_name: UserName,
    pub user_display_name: DisplayName,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TwitchEventGiftSub {
    pub anonymous: bool,

    // Total subs gifted
    pub total: i64,

    // Total gifts user has given (If not anonymous)
    pub cumulative_total: Option<i64>,
    pub tier: SubscriptionTier,

    // User details empty when cheer is anonymous
    pub user_id: Option<UserId>,
    pub user_name: Option<DisplayName>,
    pub user_display_name: Option<UserName>,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TwitchEventReSub {
    pub cumulative_months: i64,
    pub duration_months: i64,
    pub message: SubscriptionMessage,
    pub streak_months: Option<i64>,
    pub tier: SubscriptionTier,
    pub user_id: UserId,
    pub user_name: UserName,
    pub user_display_name: DisplayName,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TwitchEventChatMsg {
    pub user_id: UserId,
    pub user_name: UserName,
    pub user_display_name: UserName,
    pub message: eventsub::channel::chat::Message,
    pub cheer: Option<Cheer>,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum TwitchEvent {
    Redeem(TwitchEventRedeem),
    CheerBits(TwitchEventCheerBits),
    Follow(TwitchEventFollow),
    Sub(TwitchEventSub),
    GiftSub(TwitchEventGiftSub),
    ResubMsg(TwitchEventReSub),
    ChatMsg(TwitchEventChatMsg),
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
