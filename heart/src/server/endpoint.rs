use anyhow::Result;
use axum::Extension;
use axum::Router;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use std::sync::Arc;
use tower_http::compression::CompressionLayer;

use crate::routes;

use super::session::api_database_pool::ApiDatabasePool;
use super::session::get_api_storage;
use super::state;

use crate::app;

use dioxus::fullstack::prelude::*;
use dioxus::prelude::*;

pub async fn get_route() -> Result<Router> {
    let state = Arc::new(state::State::new().await?);

    let router = routes::get_route().await?;

    #[cfg(feature = "hot-reload")]
    state.watch();

    let index_with_state = router.layer(Extension(state.clone()));

    let router = Router::new().nest("/", index_with_state);

    let dioxus = get_app().await;
    let router = router.nest("/app", dioxus);

    let pool = get_api_storage("http://127.0.0.1:8000".to_string()).await?;
    let session_config = SessionConfig::default();
    let session_storage = SessionStore::<ApiDatabasePool>::new(Some(pool), session_config).await?;

    let session_layer = SessionLayer::new(session_storage);

    //TODO: host dioxus on the endpoints!

    Ok(router
        .layer(Extension(state.clone()))
        .layer(session_layer)
        .layer(CompressionLayer::new()))
}

async fn get_app() -> Router {
    let root: fn() -> VirtualDom = || VirtualDom::new(app::app);
    let dioxus = Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), root)
        .await;
    dioxus
}
