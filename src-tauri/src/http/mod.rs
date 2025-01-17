//! # Server
//!
//! Internal server for handling OAuth responses and serving the app overlay HTML

use crate::events::EventRecvHandle;
use crate::state::runtime_app_data::RuntimeAppDataStore;
use crate::twitch::manager::Twitch;
use crate::{database::entity::app_data::AppDataModel, storage::Storage};
use anyhow::Context;
use axum::Extension;
use sea_orm::DatabaseConnection;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tauri::AppHandle;
use tower_http::cors::CorsLayer;

mod error;
pub mod models;
pub mod routes;

pub async fn start_http_server(
    db: DatabaseConnection,
    event_handle: EventRecvHandle,
    app_handle: AppHandle,
    twitch: Twitch,
    runtime_app_data: RuntimeAppDataStore,
    storage: Storage,
) -> anyhow::Result<()> {
    let port = AppDataModel::get_http_port(&db).await?;

    // build our application with a single route
    let app = routes::router()
        .layer(Extension(db))
        .layer(Extension(event_handle))
        .layer(Extension(app_handle))
        .layer(Extension(twitch))
        .layer(Extension(runtime_app_data))
        .layer(Extension(storage))
        .layer(CorsLayer::very_permissive());

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("failed to bind http server socket")?;
    axum::serve(listener, app)
        .await
        .context("error while serving")?;

    Ok(())
}
