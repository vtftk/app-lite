use anyhow::Context;
use interlink::prelude::*;
use log::error;
use sea_orm::{prelude::DateTimeUtc, DatabaseConnection, ModelTrait};
use serde::Serialize;
use tokio::sync::{broadcast, RwLock};
use twitch_api::{
    twitch_oauth2::{AccessToken, ClientId},
    types::UserId,
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
    events::{EventMessage, ItemWithSoundIds},
    twitch::manager::{Twitch, TWITCH_CLIENT_ID},
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

#[derive(Serialize)]
pub struct TwitchCredentials {
    pub token: AccessToken,
    pub user_id: UserId,
    pub client_id: ClientId,
}

#[derive(Message)]
#[msg(rtype = "anyhow::Result<Option<TwitchCredentials>>")]
pub struct TwitchGetCredentials;

impl Handler<TwitchGetCredentials> for ScriptEventActor {
    type Response = Fr<TwitchGetCredentials>;

    fn handle(
        &mut self,
        _msg: TwitchGetCredentials,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let twitch = self.twitch.clone();
        Fr::new_box(async move {
            let token = match twitch.get_user_token().await {
                Some(value) => value,
                None => return Ok(None),
            };

            Ok(Some(TwitchCredentials {
                token: token.access_token,
                user_id: token.user_id,
                client_id: TWITCH_CLIENT_ID.to_owned(),
            }))
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

#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct EmitEventMessage {
    pub message: EventMessage,
}

impl Handler<EmitEventMessage> for ScriptEventActor {
    type Response = Mr<EmitEventMessage>;

    fn handle(&mut self, msg: EmitEventMessage, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let result = self
            .event_sender
            .send(msg.message)
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
