//! # Server
//!
//! Internal server for handling OAuth responses and serving the app overlay HTML

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use super::routes;
use crate::constants::LOCAL_SERVER_PORT;
use crate::events::EventRecvHandle;
use crate::state::app_data::AppDataStore;
use crate::state::runtime_app_data::RuntimeAppDataStore;
use crate::twitch::manager::TwitchManager;
use axum::Extension;
use tauri::AppHandle;
use tower_http::cors::CorsLayer;

pub async fn start(
    event_handles: EventRecvHandle,
    app_handle: AppHandle,
    twitch_manager: Arc<TwitchManager>,
    app_data: AppDataStore,
    runtime_app_data: RuntimeAppDataStore,
) {
    // build our application with a single route
    let app = routes::router()
        .layer(Extension(event_handles))
        .layer(Extension(app_handle))
        .layer(Extension(twitch_manager))
        .layer(Extension(app_data))
        .layer(Extension(runtime_app_data))
        .layer(CorsLayer::very_permissive());

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, LOCAL_SERVER_PORT));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
