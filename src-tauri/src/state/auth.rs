use std::sync::Arc;

use tauri::async_runtime::Mutex;
use twitch_api::twitch_oauth2::UserToken;

use crate::commands::auth;

/// Auth state that can be shared between threads
#[derive(Default, Clone)]
pub struct SharedAuthState(Arc<Mutex<AuthState>>);

impl SharedAuthState {
    /// Sets the authenticated token
    pub async fn set_authenticated(&self, token: UserToken) {
        let lock = &mut *self.0.lock().await;
        *lock = AuthState::Authenticated { token }
    }

    /// Checks if the state is currently authenticated
    pub async fn is_authenticated(&self) -> bool {
        let state = &*self.0.lock().await;
        matches!(state, AuthState::NotAuthenticated)
    }
}

#[derive(Debug, Default)]
#[allow(clippy::large_enum_variant)]
pub enum AuthState {
    /// Not yet authenticated
    #[default]
    NotAuthenticated,

    /// Authenticated
    Authenticated {
        /// Currently active user authentication token
        token: UserToken,
    },
}
