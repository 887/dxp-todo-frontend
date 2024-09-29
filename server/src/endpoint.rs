use crate::error::error_handling_middleware;
use crate::error::ErrorMiddleware;
use crate::routes;
use crate::session;
use crate::state;
use anyhow::{Context, Result};
use axum::middleware::{from_fn, Next};
use axum::response::Response;
use axum::routing::get;
use axum::Extension;
use axum::Router;
use std::sync::Arc;
use tower_http::compression::CompressionLayer;

pub async fn get_route() -> Result<Router> {
    let state = Arc::new(state::State::new().await?);

    let api = std::env::var("API").context("API is not set")?;
    let session_storage = session::get_api_storage(api).await?;
    let session_middleware = session::get_session_middleware(session_storage)?;

    let error_middleware = ErrorMiddleware {
        templates: state.templates,
    };

    let index = routes::get_route().await?;

    #[cfg(feature = "hot-reload")]
    state.watch();

    let index_with_state = index.layer(Extension(state.clone()));

    let route = Router::new().nest("/", index_with_state);

    Ok(route
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            error_handling_middleware,
        ))
        .layer(session_middleware)
        .layer(CompressionLayer::new()))
}
