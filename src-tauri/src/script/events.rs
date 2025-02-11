use anyhow::Context;
use interlink::prelude::*;
use log::error;
use sea_orm::{prelude::DateTimeUtc, DatabaseConnection, ModelTrait};
use tokio::sync::{broadcast, RwLock};
use twitch_api::{
    helix::channels::Follower,
    types::{MsgId, UserId},
};
use uuid::Uuid;

use crate::{
    database::entity::{
        command_logs::{CommandLogsModel, CreateCommandLog},
        event_logs::{CreateEventLog, EventLogsModel},
        items::ItemModel,
        items_sounds::SoundType,
        key_value::{CreateKeyValue, KeyValueModel, KeyValueType},
        shared::LoggingLevelDb,
        sounds::SoundModel,
    },
    events::{EventMessage, ItemWithSoundIds, ItemsWithSounds, ThrowItemConfig, ThrowItemMessage},
    twitch::{manager::Twitch, models::TwitchUser},
};

use super::runtime::RuntimeExecutionContext;

/// Current global instance of the script event actor
static GLOBAL_SCRIPT_EVENT_ACTOR: RwLock<Option<Link<ScriptEventActor>>> = RwLock::const_new(None);

pub async fn init_global_script_event_actor(actor: ScriptEventActor) {
    let link = actor.start();

    // Can block here, initialization will never have any other writers so won't be blocked
    *GLOBAL_SCRIPT_EVENT_ACTOR.write().await = Some(link);
}

pub async fn global_script_event<M>(msg: M) -> anyhow::Result<M::Response>
where
    M: Message,
    ScriptEventActor: Handler<M>,
{
    let link = &*GLOBAL_SCRIPT_EVENT_ACTOR.read().await;

    let link = link
        .as_ref()
        .context("global script event actor not initialized")?;

    let response = link.send(msg).await.context("failed to send message")?;

    Ok(response)
}

/// Actor responsible for handling script operations that
/// require accessing other portions of the app such as
/// interacting with twitch, accessing app data, using the
/// KV store etc etc
#[derive(Service)]
pub struct ScriptEventActor {
    /// Sender handle for submitting event messages
    event_sender: broadcast::Sender<EventMessage>,

    /// Access to the database
    db: DatabaseConnection,

    /// Access to the twitch manager
    twitch: Twitch,
}

impl ScriptEventActor {
    pub fn new(
        event_sender: broadcast::Sender<EventMessage>,
        db: DatabaseConnection,
        twitch: Twitch,
    ) -> Self {
        Self {
            event_sender,
            db,
            twitch,
        }
    }
}

/// Message to trigger sending a message to Twitch chat
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct TwitchSendChat {
    pub message: String,
}

impl Handler<TwitchSendChat> for ScriptEventActor {
    type Response = Fr<TwitchSendChat>;

    fn handle(&mut self, msg: TwitchSendChat, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            _ = twitch.send_chat_message(&msg.message).await?;
            Ok(())
        })
    }
}

/// Message to trigger deleting a message from Twitch chat
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct TwitchDeleteChatMessage {
    pub message_id: MsgId,
}

impl Handler<TwitchDeleteChatMessage> for ScriptEventActor {
    type Response = Fr<TwitchDeleteChatMessage>;

    fn handle(
        &mut self,
        msg: TwitchDeleteChatMessage,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            twitch.delete_chat_message(msg.message_id).await?;
            Ok(())
        })
    }
}

/// Message to trigger deleting all messages from Twitch chat
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct TwitchDeleteAllChatMessages;

impl Handler<TwitchDeleteAllChatMessages> for ScriptEventActor {
    type Response = Fr<TwitchDeleteAllChatMessages>;

    fn handle(
        &mut self,
        _msg: TwitchDeleteAllChatMessages,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            twitch.delete_all_chat_messages().await?;
            Ok(())
        })
    }
}

/// Message to trigger creating a twitch stream marker
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct TwitchCreateStreamMarker {
    pub description: Option<String>,
}

impl Handler<TwitchCreateStreamMarker> for ScriptEventActor {
    type Response = Fr<TwitchCreateStreamMarker>;

    fn handle(
        &mut self,
        msg: TwitchCreateStreamMarker,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            twitch.create_stream_marker(msg.description).await?;
            Ok(())
        })
    }
}

/// Message to check if a user is a moderator for a twitch channel
#[derive(Message)]
#[msg(rtype = "anyhow::Result<bool>")]
pub struct TwitchIsMod {
    pub user_id: UserId,
}

impl Handler<TwitchIsMod> for ScriptEventActor {
    type Response = Fr<TwitchIsMod>;

    fn handle(&mut self, msg: TwitchIsMod, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            let mods = twitch.get_moderator_list().await?;
            Ok(mods.iter().any(|vip| vip.user_id == msg.user_id))
        })
    }
}

/// Message to check if a user is a VIP for a twitch channel
#[derive(Message)]
#[msg(rtype = "anyhow::Result<bool>")]
pub struct TwitchIsVip {
    pub user_id: UserId,
}

impl Handler<TwitchIsVip> for ScriptEventActor {
    type Response = Fr<TwitchIsVip>;

    fn handle(&mut self, msg: TwitchIsVip, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            let vips = twitch.get_vip_list().await?;
            Ok(vips.iter().any(|vip| vip.user_id == msg.user_id))
        })
    }
}

/// Message to trigger sending an announcement message to Twitch chat
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct TwitchSendChatAnnouncement {
    pub message: String,
    pub color: String,
}

impl Handler<TwitchSendChatAnnouncement> for ScriptEventActor {
    type Response = Fr<TwitchSendChatAnnouncement>;

    fn handle(
        &mut self,
        msg: TwitchSendChatAnnouncement,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            _ = twitch
                .send_chat_announcement_message(msg.message, msg.color)
                .await?;
            Ok(())
        })
    }
}

/// Message to get a twitch user using their username
#[derive(Message)]
#[msg(rtype = "anyhow::Result<Option<TwitchUser>>")]
pub struct TwitchGetUserByUsername {
    pub username: String,
}

impl Handler<TwitchGetUserByUsername> for ScriptEventActor {
    type Response = Fr<TwitchGetUserByUsername>;

    fn handle(
        &mut self,
        msg: TwitchGetUserByUsername,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            let user = twitch.get_user_by_username(&msg.username).await?;
            Ok(user)
        })
    }
}
/// Message to get a twitch user using their username
#[derive(Message)]
#[msg(rtype = "anyhow::Result<Option<Follower>>")]
pub struct TwitchGetFollower {
    pub user_id: UserId,
}

impl Handler<TwitchGetFollower> for ScriptEventActor {
    type Response = Fr<TwitchGetFollower>;

    fn handle(
        &mut self,
        msg: TwitchGetFollower,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            let user = twitch.get_follower_by_id(msg.user_id).await?;
            Ok(user)
        })
    }
}

/// Message to send a shoutout to a user
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct TwitchSendShoutout {
    pub user_id: UserId,
}

impl Handler<TwitchSendShoutout> for ScriptEventActor {
    type Response = Fr<TwitchSendShoutout>;

    fn handle(
        &mut self,
        msg: TwitchSendShoutout,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            _ = twitch.send_shoutout(msg.user_id).await?;
            Ok(())
        })
    }
}

/// Message to set a key value on the key value store
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct KvSet {
    pub key: String,
    pub ty: KeyValueType,
    pub value: String,
}

impl Handler<KvSet> for ScriptEventActor {
    type Response = Fr<KvSet>;

    fn handle(&mut self, msg: KvSet, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let db = self.db.clone();
        Fr::new_box(async move {
            KeyValueModel::create(
                &db,
                CreateKeyValue {
                    key: msg.key,
                    value: msg.value,
                    ty: msg.ty,
                },
            )
            .await?;

            Ok(())
        })
    }
}

/// Message to remove a key value from the key value store
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct KvRemove {
    pub key: String,
}

impl Handler<KvRemove> for ScriptEventActor {
    type Response = Fr<KvRemove>;

    fn handle(&mut self, msg: KvRemove, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let db = self.db.clone();
        Fr::new_box(async move {
            if let Some(key_value) = KeyValueModel::get_by_key(&db, &msg.key).await? {
                key_value.delete(&db).await?;
            }

            Ok(())
        })
    }
}

/// Message to get a value from the KV store
#[derive(Message)]
#[msg(rtype = "anyhow::Result<Option<String>>")]
pub struct KvGet {
    pub key: String,
}

impl Handler<KvGet> for ScriptEventActor {
    type Response = Fr<KvGet>;

    fn handle(&mut self, msg: KvGet, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let db = self.db.clone();
        Fr::new_box(async move {
            let key_value = KeyValueModel::get_by_key(&db, &msg.key).await?;
            let value = key_value.map(|value| value.value);
            Ok(value)
        })
    }
}

/// Message to get sounds with a matching name
#[derive(Message)]
#[msg(rtype = "anyhow::Result<Vec<SoundModel>>")]
pub struct GetSoundsByNames {
    pub names: Vec<String>,
    pub ignore_case: bool,
}

impl Handler<GetSoundsByNames> for ScriptEventActor {
    type Response = Fr<GetSoundsByNames>;

    fn handle(&mut self, msg: GetSoundsByNames, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let db = self.db.clone();
        Fr::new_box(async move {
            let sounds = SoundModel::get_by_names(&db, &msg.names, msg.ignore_case).await?;
            Ok(sounds)
        })
    }
}

/// Message to get a sound by ID
#[derive(Message)]
#[msg(rtype = "anyhow::Result<Vec<SoundModel>>")]
pub struct GetSoundsByIDs {
    pub ids: Vec<Uuid>,
}

impl Handler<GetSoundsByIDs> for ScriptEventActor {
    type Response = Fr<GetSoundsByIDs>;

    fn handle(&mut self, msg: GetSoundsByIDs, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let db = self.db.clone();
        Fr::new_box(async move {
            let sound = SoundModel::get_by_ids(&db, &msg.ids).await?;
            Ok(sound)
        })
    }
}

/// Message to get sounds with a matching name
#[derive(Message)]
#[msg(rtype = "anyhow::Result<Vec<ItemWithSoundIds>>")]
pub struct GetItemsByNames {
    pub names: Vec<String>,
    pub ignore_case: bool,
}

impl Handler<GetItemsByNames> for ScriptEventActor {
    type Response = Fr<GetItemsByNames>;

    fn handle(&mut self, msg: GetItemsByNames, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let db = self.db.clone();
        Fr::new_box(async move {
            let items: Vec<ItemWithSoundIds> =
                ItemModel::get_by_names_with_sounds(&db, &msg.names, msg.ignore_case)
                    .await?
                    .into_iter()
                    .map(|(item, sounds)| {
                        let mut impact_sound_ids = Vec::new();
                        let mut windup_sound_ids = Vec::new();

                        for sound in sounds {
                            match sound.sound_type {
                                SoundType::Impact => impact_sound_ids.push(sound.sound_id),
                                SoundType::Windup => windup_sound_ids.push(sound.sound_id),
                            }
                        }

                        ItemWithSoundIds {
                            item,
                            impact_sound_ids,
                            windup_sound_ids,
                        }
                    })
                    .collect();

            Ok(items)
        })
    }
}

/// Message to get a sound by ID
#[derive(Message)]
#[msg(rtype = "anyhow::Result<Vec<ItemWithSoundIds>>")]
pub struct GetItemsByIDs {
    pub ids: Vec<Uuid>,
}

impl Handler<GetItemsByIDs> for ScriptEventActor {
    type Response = Fr<GetItemsByIDs>;

    fn handle(&mut self, msg: GetItemsByIDs, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let db = self.db.clone();
        Fr::new_box(async move {
            let items: Vec<ItemWithSoundIds> = ItemModel::get_by_ids_with_sounds(&db, &msg.ids)
                .await?
                .into_iter()
                .map(|(item, sounds)| {
                    let mut impact_sound_ids = Vec::new();
                    let mut windup_sound_ids = Vec::new();

                    for sound in sounds {
                        match sound.sound_type {
                            SoundType::Impact => impact_sound_ids.push(sound.sound_id),
                            SoundType::Windup => windup_sound_ids.push(sound.sound_id),
                        }
                    }

                    ItemWithSoundIds {
                        item,
                        impact_sound_ids,
                        windup_sound_ids,
                    }
                })
                .collect();

            Ok(items)
        })
    }
}

/// Message to throw items
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct ThrowItems {
    pub items: ItemsWithSounds,
    pub config: ThrowItemConfig,
}

impl Handler<ThrowItems> for ScriptEventActor {
    type Response = Mr<ThrowItems>;

    fn handle(&mut self, msg: ThrowItems, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let result = self
            .event_sender
            .send(EventMessage::ThrowItem(ThrowItemMessage {
                items: msg.items,
                config: msg.config,
            }))
            .context("event receiver was closed");

        Mr(result.map(|_| ()))
    }
}

#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct TriggerHotkey {
    pub hotkey_id: String,
}

impl Handler<TriggerHotkey> for ScriptEventActor {
    type Response = Mr<TriggerHotkey>;

    fn handle(&mut self, msg: TriggerHotkey, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let result = self
            .event_sender
            .send(EventMessage::TriggerHotkey {
                hotkey_id: msg.hotkey_id,
            })
            .context("event receiver was closed");

        Mr(result.map(|_| ()))
    }
}

#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct TriggerHotkeyByName {
    pub hotkey_name: String,
    pub ignore_case: bool,
}

impl Handler<TriggerHotkeyByName> for ScriptEventActor {
    type Response = Mr<TriggerHotkeyByName>;

    fn handle(
        &mut self,
        msg: TriggerHotkeyByName,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let result = self
            .event_sender
            .send(EventMessage::TriggerHotkeyByName {
                hotkey_name: msg.hotkey_name,
                ignore_case: msg.ignore_case,
            })
            .context("event receiver was closed");

        Mr(result.map(|_| ()))
    }
}

/// Message to play a sound
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct PlaySound {
    pub config: SoundModel,
}

impl Handler<PlaySound> for ScriptEventActor {
    type Response = Mr<PlaySound>;

    fn handle(&mut self, msg: PlaySound, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let result = self
            .event_sender
            .send(EventMessage::PlaySound { config: msg.config })
            .context("event receiver was closed");

        Mr(result.map(|_| ()))
    }
}

/// Message to play a sequence of sounds in order
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct PlaySoundSeq {
    pub configs: Vec<SoundModel>,
}

impl Handler<PlaySoundSeq> for ScriptEventActor {
    type Response = Mr<PlaySoundSeq>;

    fn handle(&mut self, msg: PlaySoundSeq, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let result = self
            .event_sender
            .send(EventMessage::PlaySoundSeq {
                configs: msg.configs,
            })
            .context("event receiver was closed");

        Mr(result.map(|_| ()))
    }
}

#[derive(Message)]
#[msg(rtype = "()")]
pub struct LogPersistEvent {
    pub ctx: RuntimeExecutionContext,
    pub level: LoggingLevelDb,
    pub message: String,
    pub created_at: DateTimeUtc,
}

impl Handler<LogPersistEvent> for ScriptEventActor {
    type Response = Fr<LogPersistEvent>;

    fn handle(&mut self, msg: LogPersistEvent, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let db = self.db.clone();
        Fr::new_box(async move {
            match msg.ctx {
                RuntimeExecutionContext::Event { event_id } => {
                    if let Err(err) = EventLogsModel::create(
                        &db,
                        CreateEventLog {
                            event_id,
                            level: msg.level,
                            message: msg.message,
                            created_at: msg.created_at,
                        },
                    )
                    .await
                    {
                        error!("failed to persist script log: {:?}", err);
                    }
                }
                RuntimeExecutionContext::Command { command_id } => {
                    if let Err(err) = CommandLogsModel::create(
                        &db,
                        CreateCommandLog {
                            command_id,
                            level: msg.level,
                            message: msg.message,
                            created_at: msg.created_at,
                        },
                    )
                    .await
                    {
                        error!("failed to persist command log: {:?}", err);
                    }
                }
            };
        })
    }
}
