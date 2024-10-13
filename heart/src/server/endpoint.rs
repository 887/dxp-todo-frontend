use anyhow::{Context, Result};
use axum::middleware::{from_fn, Next};
use axum::response::Response;
use axum::routing::get;
use axum::Extension;
use axum::Router;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use std::sync::Arc;
use tower_http::compression::CompressionLayer;

use crate::routes;

use super::session::api_database_pool::ApiDatabasePool;
use super::session::get_api_storage;
use super::state;

pub async fn get_route() -> Result<Router> {
    let state = Arc::new(state::State::new().await?);

    let api = std::env::var("API").context("API is not set")?;

    let app = routes::get_route().await?;

    #[cfg(feature = "hot-reload")]
    state.watch();

    let index_with_state = app.layer(Extension(state.clone()));

    let route = Router::new().nest("/", index_with_state);

    let pool = get_api_storage("http://127.0.0.1:8000".to_string()).await?;
    let session_config = SessionConfig::default();
    let session_storage = SessionStore::<ApiDatabasePool>::new(Some(pool), session_config).await?;

    let session_layer = SessionLayer::new(session_storage);

    Ok(route
        .layer(Extension(state.clone()))
        .layer(session_layer)
        .layer(CompressionLayer::new()))
}
