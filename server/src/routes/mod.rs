use anyhow::Result;

use poem::{get, IntoEndpoint, Route};

mod index;
mod test;

pub(crate) async fn get_route() -> Result<impl IntoEndpoint> {
    let route = Route::new().at("/", get(index::index));

    Ok(route.nest("/test", test::get_route().await?))
}
