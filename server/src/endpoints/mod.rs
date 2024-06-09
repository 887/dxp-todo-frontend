use anyhow::{Context, Result};
use error::ErrorMiddleware;
use poem::middleware::Compression;
use poem::{Endpoint, EndpointExt, Route};

mod error;
mod i18n;
mod routes;
mod session;
mod state;
mod templates;

#[cfg(feature = "hot-reload")]
mod watcher;

pub async fn get_route() -> Result<impl Endpoint> {
    let state = state::State::new()?;

    let api = std::env::var("API").context("API is not set")?;
    let session_storage = session::get_api_storage(api).await?;
    let session_middleware = session::get_session_middleware(session_storage)?;
    let error_middleware = ErrorMiddleware {
        templates: state.templates,
    };

    let index = routes::get_route().await?;

    #[cfg(feature = "hot-reload")]
    state.watch();

    let index_with_state = index.data(state);

    let route = Route::new().nest("/", index_with_state); //routers need to be nested

    Ok(route
        .with(error_middleware)
        .with(session_middleware)
        .with(Compression::new()))
}
