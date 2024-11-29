use std::sync::Arc;

use anyhow::{anyhow, Context};
use futures::TryStreamExt;
use log::{debug, error};
use tauri::{AppHandle, Emitter};
use tokio::{
    join,
    sync::{broadcast, RwLock},
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
    helix::{channels::Vip, moderation::Moderator, points::CustomReward},
    twitch_oauth2::UserToken,
    types::{DisplayName, RedemptionId, SubscriptionTier, UserId, UserName},
    HelixClient,
};

use super::websocket::WebsocketClient;

pub struct TwitchManager {
    pub helix_client: HelixClient<'static, reqwest::Client>,
    state: RwLock<TwitchManagerState>,
    tx: broadcast::Sender<TwitchEvent>,
    app_handle: AppHandle,
}

pub struct TwitchManagerStateAuthenticated {
    // Token for the authenticated user
    token: UserToken,
    // Currently active websocket connection
    _websocket: WebsocketManagedTask,

    // List of available rewards
    rewards: Option<Arc<[CustomReward]>>,
    // Current loaded list of moderators
    moderators: Option<Arc<[Moderator]>>,
    // Current loaded list of vips
    vips: Option<Arc<[Vip]>>,
}

#[derive(Default)]
#[allow(clippy::large_enum_variant)]
enum TwitchManagerState {
    // Twitch is not yet authenticated
    #[default]
    Initial,
    // Twitch is authenticated
    Authenticated(TwitchManagerStateAuthenticated),
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
        let lock = &*self.state.read().await;
        matches!(lock, TwitchManagerState::Authenticated { .. })
    }

    pub async fn get_user_token(&self) -> Option<UserToken> {
        let lock = &*self.state.read().await;
        match lock {
            TwitchManagerState::Initial => None,
            TwitchManagerState::Authenticated(state) => Some(state.token.clone()),
        }
    }

    pub async fn set_authenticated(self: &Arc<Self>, token: UserToken) {
        {
            let lock = &mut *self.state.write().await;

            let websocket =
                WebsocketManagedTask::create(self.clone(), self.tx.clone(), token.clone());

            *lock = TwitchManagerState::Authenticated(TwitchManagerStateAuthenticated {
                token,
                _websocket: websocket,
                moderators: None,
                vips: None,
                rewards: None,
            });
        }

        // Tell the app we are authenticated
        _ = self.app_handle.emit("authenticated", ());

        // Load initial moderator and VIP lists
        let (rewards_result, vips_result, mods_result) = join!(
            self.get_rewards_list(),
            self.get_vip_list(),
            self.get_moderator_list()
        );

        if let Err(err) = rewards_result {
            error!("failed to load rewards: {:?}", err);
        }

        if let Err(err) = vips_result {
            error!("failed to load vips: {:?}", err);
        }

        if let Err(err) = mods_result {
            error!("failed to load mods: {:?}", err);
        }
    }

    pub async fn reset(&self) {
        {
            let lock = &mut *self.state.write().await;
            *lock = TwitchManagerState::Initial;
        }

        // Tell the app we are authenticated
        _ = self.app_handle.emit("logout", ());
    }

    pub async fn get_moderator_list(&self) -> anyhow::Result<Arc<[Moderator]>> {
        // First attempt to read existing list
        {
            let state = &*self.state.read().await;
            match state {
                TwitchManagerState::Initial => return Err(anyhow!("not authenticated")),
                TwitchManagerState::Authenticated(state) => {
                    if let Some(moderators) = state.moderators.as_ref() {
                        return Ok(moderators.clone());
                    }
                }
            }
        }

        let moderators = self.request_moderator_list().await?;
        let moderators: Arc<[Moderator]> = moderators.into();

        // Write new list
        let state = &mut *self.state.write().await;
        match state {
            TwitchManagerState::Initial => Err(anyhow!("not authenticated")),
            TwitchManagerState::Authenticated(state) => {
                state.moderators = Some(moderators.clone());
                Ok(moderators)
            }
        }
    }

    pub async fn get_vip_list(&self) -> anyhow::Result<Arc<[Vip]>> {
        // First attempt to read existing list
        {
            let state = &*self.state.read().await;
            match state {
                TwitchManagerState::Initial => return Err(anyhow!("not authenticated")),
                TwitchManagerState::Authenticated(state) => {
                    if let Some(vips) = state.vips.as_ref() {
                        return Ok(vips.clone());
                    }
                }
            }
        }
        let vips = self.request_vip_list().await?;

        // Write new list
        let state = &mut *self.state.write().await;
        match state {
            TwitchManagerState::Initial => Err(anyhow!("not authenticated")),
            TwitchManagerState::Authenticated(state) => {
                let vips: Arc<[Vip]> = vips.into();
                state.vips = Some(vips.clone());

                Ok(vips)
            }
        }
    }

    pub async fn get_rewards_list(&self) -> anyhow::Result<Arc<[CustomReward]>> {
        // First attempt to read existing list
        {
            let state = &*self.state.read().await;
            match state {
                TwitchManagerState::Initial => return Err(anyhow!("not authenticated")),
                TwitchManagerState::Authenticated(state) => {
                    if let Some(rewards) = state.rewards.as_ref() {
                        return Ok(rewards.clone());
                    }
                }
            }
        }

        let rewards = self.request_rewards_list().await?;
        let rewards: Arc<[CustomReward]> = rewards.into();

        // Write new list
        let state = &mut *self.state.write().await;
        match state {
            TwitchManagerState::Initial => Err(anyhow!("not authenticated")),
            TwitchManagerState::Authenticated(state) => {
                state.rewards = Some(rewards.clone());

                Ok(rewards)
            }
        }
    }

    pub async fn reload_moderator_list(&self) {
        debug!("reloading mods list");
        self.clear_moderator_list().await;
        _ = self.get_moderator_list().await;
    }

    pub async fn reload_vip_list(&self) {
        debug!("reloading vip list");
        self.clear_vip_list().await;
        _ = self.get_vip_list().await;
    }

    pub async fn reload_rewards_list(&self) {
        debug!("reloading rewards list");
        self.clear_rewards_list().await;
        _ = self.get_rewards_list().await;
    }

    async fn clear_moderator_list(&self) {
        let state = &mut *self.state.write().await;

        // Use existing list
        if let TwitchManagerState::Authenticated(inner_state) = &mut *state {
            inner_state.moderators = None;
        }
    }

    async fn clear_vip_list(&self) {
        let state = &mut *self.state.write().await;

        // Use existing list
        if let TwitchManagerState::Authenticated(inner_state) = &mut *state {
            inner_state.vips = None;
        }
    }

    async fn clear_rewards_list(&self) {
        let state = &mut *self.state.write().await;

        // Use existing list
        if let TwitchManagerState::Authenticated(inner_state) = &mut *state {
            inner_state.rewards = None;
        }
    }

    async fn request_moderator_list(&self) -> anyhow::Result<Vec<Moderator>> {
        let user_token = self.get_user_token().await.context("not authenticated")?;
        let user_id = user_token.user_id.clone();

        let moderators: Vec<Moderator> = self
            .helix_client
            .get_moderators_in_channel_from_id(user_id, &user_token)
            .try_collect()
            .await?;

        Ok(moderators)
    }

    async fn request_vip_list(&self) -> anyhow::Result<Vec<Vip>> {
        let user_token = self.get_user_token().await.context("not authenticated")?;
        let user_id = user_token.user_id.clone();

        let moderators: Vec<Vip> = self
            .helix_client
            .get_vips_in_channel(user_id, &user_token)
            .try_collect()
            .await?;

        Ok(moderators)
    }

    async fn request_rewards_list(&self) -> anyhow::Result<Vec<CustomReward>> {
        let user_token = self.get_user_token().await.context("not authenticated")?;
        let user_id = user_token.user_id.clone();
        let rewards = self
            .helix_client
            .get_all_custom_rewards(user_id, false, &user_token)
            .await?;

        Ok(rewards)
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TwitchEventUser {
    pub user_id: UserId,
    pub user_name: UserName,
    pub user_display_name: DisplayName,
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
    pub user_name: Option<UserName>,
    pub user_display_name: Option<DisplayName>,
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
    pub user_display_name: DisplayName,
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

    ModeratorsChanged,
    VipsChanged,
    RewardsChanged,
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
