use crate::database::entity::shared::{DbResult, MinimumRequireRole};
use crate::database::entity::{ItemModel, SoundModel};
use crate::state::app_data::{ItemWithImpactSoundIds, ThrowableConfig};
use crate::twitch::manager::TwitchManager;
use futures::future::BoxFuture;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use log::error;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tokio::time::Instant;
use tokio::try_join;
use twitch_api::types::UserId;
use uuid::Uuid;

#[derive(Default)]
pub struct EventsState {
    // TODO: MOVE THIS INTO THE DATABASE
    // Last execution time per event
    pub event_last_execution: HashMap<Uuid, Instant>,
}

#[derive(Default, Clone)]
pub struct EventsStateShared {
    inner: Arc<RwLock<EventsState>>,
}

impl EventsStateShared {
    pub async fn is_cooldown_elapsed(&self, uuid: &Uuid, cooldown: Duration) -> bool {
        let now = Instant::now();

        let inner = &*self.inner.read().await;
        if let Some(last_instant) = inner.event_last_execution.get(uuid) {
            let elapsed = now.duration_since(*last_instant);
            elapsed > cooldown
        } else {
            true
        }
    }

    pub async fn set_last_executed(&self, uuid: &Uuid) {
        let now = Instant::now();
        let inner = &mut *self.inner.write().await;
        inner.event_last_execution.insert(*uuid, now);
    }
}

pub async fn assert_required_role(
    twitch_manager: &TwitchManager,
    user_id: Option<UserId>,
    required_role: &MinimumRequireRole,
) -> bool {
    match required_role {
        MinimumRequireRole::None => true,
        MinimumRequireRole::Vip => {
            let user = match user_id {
                Some(user) => user,
                None => return false,
            };

            // User is the broadcaster
            if twitch_manager
                .get_user_token()
                .await
                .is_some_and(|value| value.user_id == user)
            {
                return true;
            }

            let (vips, mods) = match try_join!(
                twitch_manager.get_vip_list(),
                twitch_manager.get_moderator_list()
            ) {
                Ok(value) => value,
                Err(_) => return false,
            };

            vips.iter().any(|vip| vip.user_id == user)
                || mods.iter().any(|mods| mods.user_id == user)
        }
        MinimumRequireRole::Mod => {
            let user = match user_id {
                Some(user) => user,
                None => return false,
            };

            // User is the broadcaster
            if twitch_manager
                .get_user_token()
                .await
                .is_some_and(|value| value.user_id == user)
            {
                return true;
            }

            let mods = match twitch_manager.get_moderator_list().await {
                Ok(value) => value,
                Err(_) => {
                    return false;
                }
            };

            mods.iter().any(|mods| mods.user_id == user)
        }
    }
}

pub async fn create_throwable_config(
    db: &DatabaseConnection,
    items: Vec<ItemModel>,
) -> anyhow::Result<ThrowableConfig> {
    // Find all the referenced sounds
    let mut futures = items
        .into_iter()
        .map(
            |item| -> BoxFuture<'_, DbResult<(ItemWithImpactSoundIds, Vec<SoundModel>)>> {
                Box::pin(async move {
                    let sounds = item.get_impact_sounds(db).await?;

                    let impact_sound_ids = sounds.iter().map(|sound| sound.id).collect();
                    Ok((
                        ItemWithImpactSoundIds {
                            item,
                            impact_sound_ids,
                        },
                        sounds,
                    ))
                })
            },
        )
        .collect::<FuturesUnordered<_>>();

    let mut items = Vec::new();
    let mut impact_sounds = Vec::new();

    while let Some(result) = futures.next().await {
        match result {
            Ok((item, mut sounds)) => {
                items.push(item);
                impact_sounds.append(&mut sounds);
            }
            Err(err) => {
                error!("error loading impact sounds: {:?}", err);
            }
        }
    }

    Ok(ThrowableConfig {
        items,
        impact_sounds,
    })
}
