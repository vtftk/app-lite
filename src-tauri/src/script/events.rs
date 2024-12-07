use anyhow::Context;
use interlink::prelude::*;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use twitch_api::types::UserId;

use crate::{
    events::EventMessage,
    state::app_data::{AppDataStore, SoundConfig},
    tts::{
        tts_monster_generate, tts_monster_generate_parsed, tts_monster_get_voices, GenerateRequest,
        GenerateResponse, Voice,
    },
    twitch::manager::TwitchManager,
};

use super::kv::KVStore;

/// Current global instance of the script event actor
pub static GLOBAL_SCRIPT_EVENT_ACTOR: RwLock<Option<Link<ScriptEventActor>>> =
    RwLock::const_new(None);

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
    /// App data store access
    app_data: AppDataStore,

    /// Sender handle for submitting event messages
    event_sender: broadcast::Sender<EventMessage>,

    /// Access to the KV store
    kv_store: KVStore,

    /// Access to the twitch manager
    twitch_manager: Arc<TwitchManager>,
}

impl ScriptEventActor {
    pub fn new(
        app_data: AppDataStore,
        event_sender: broadcast::Sender<EventMessage>,
        kv_store: KVStore,
        twitch_manager: Arc<TwitchManager>,
    ) -> Self {
        Self {
            app_data,
            event_sender,
            kv_store,
            twitch_manager,
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
        let twitch_manager = self.twitch_manager.clone();
        Fr::new_box(async move {
            _ = twitch_manager.send_chat_message(msg.message).await?;
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
        let twitch_manager = self.twitch_manager.clone();
        Fr::new_box(async move {
            let mods = twitch_manager.get_moderator_list().await?;
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
        let twitch_manager = self.twitch_manager.clone();
        Fr::new_box(async move {
            let vips = twitch_manager.get_vip_list().await?;
            Ok(vips.iter().any(|vip| vip.user_id == msg.user_id))
        })
    }
}

/// Message to set a key value on the key value store
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct KvSet {
    pub key: String,
    pub value: String,
}

impl Handler<KvSet> for ScriptEventActor {
    type Response = Fr<KvSet>;

    fn handle(&mut self, msg: KvSet, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let kv_store = self.kv_store.clone();
        Fr::new_box(async move {
            kv_store.set(&msg.key, msg.value).await?;
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
        let kv_store = self.kv_store.clone();
        Fr::new_box(async move {
            kv_store.remove(&msg.key).await?;
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
        let kv_store = self.kv_store.clone();
        Fr::new_box(async move {
            let value = kv_store.get(&msg.key).await;
            Ok(value)
        })
    }
}

/// Message to play a sound
#[derive(Message)]
#[msg(rtype = "anyhow::Result<()>")]
pub struct PlaySound {
    pub config: SoundConfig,
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
    pub configs: Vec<SoundConfig>,
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

/// Message to get the list of available TTS voices
#[derive(Message)]
#[msg(rtype = "anyhow::Result<Vec<Voice>>")]
pub struct TTSGetVoices;

impl Handler<TTSGetVoices> for ScriptEventActor {
    type Response = Fr<TTSGetVoices>;

    fn handle(&mut self, _msg: TTSGetVoices, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let app_data: AppDataStore = self.app_data.clone();
        Fr::new_box(tts_monster_get_voices(app_data))
    }
}

/// Message to generate a TTS message
#[derive(Message)]
#[msg(rtype = "anyhow::Result<GenerateResponse>")]
pub struct TTSGenerate {
    pub request: GenerateRequest,
}

impl Handler<TTSGenerate> for ScriptEventActor {
    type Response = Fr<TTSGenerate>;

    fn handle(&mut self, msg: TTSGenerate, _ctx: &mut ServiceContext<Self>) -> Self::Response {
        let app_data: AppDataStore = self.app_data.clone();
        Fr::new_box(tts_monster_generate(app_data, msg.request))
    }
}

/// Message to generate a TTS message from a message that
/// is first parsed to determine which voices to use
#[derive(Message)]
#[msg(rtype = "anyhow::Result<Vec<String>>")]
pub struct TTSGenerateParsed {
    pub message: String,
}

impl Handler<TTSGenerateParsed> for ScriptEventActor {
    type Response = Fr<TTSGenerateParsed>;

    fn handle(
        &mut self,
        msg: TTSGenerateParsed,
        _ctx: &mut ServiceContext<Self>,
    ) -> Self::Response {
        let app_data: AppDataStore = self.app_data.clone();
        Fr::new_box(tts_monster_generate_parsed(app_data, msg.message))
    }
}
