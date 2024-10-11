mod hello;
mod session;

use anyhow::Result;

use poem::{get, IntoEndpoint, Route};

pub(crate) async fn get_route() -> Result<impl IntoEndpoint> {
    Ok(Route::new()
        .at("/hello", get(hello::hello))
        .at("/session", get(session::session)))
}
