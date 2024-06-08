mod state;

use anyhow::{Context, Result};
use error::ErrorMiddleware;
use poem::middleware::Compression;
use poem::{Endpoint, EndpointExt, Route};

mod error;
mod routes;
mod session;
mod templates;

pub async fn get_route() -> Result<impl Endpoint> {
    let templates = templates::get_templates();
    #[cfg(feature = "hot-reload")]
    templates::watch_directory(templates::TEMPLATE_DIR, templates);

    let api = std::env::var("API").context("API is not set")?;

    let session_storage = session::get_api_storage(api).await?;
    let session_middleware = session::get_session_middleware(session_storage)?;

    let index = routes::get_route().await?;

    let state = state::State { templates };

    let index_with_state = index.data(state);

    let route = Route::new().nest("/", index_with_state); //routers need to be nested

    let error_middleware = ErrorMiddleware { templates };
    Ok(route
        .with(error_middleware)
        .with(session_middleware)
        .with(Compression::new()))
}
