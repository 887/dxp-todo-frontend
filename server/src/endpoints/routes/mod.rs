use anyhow::Result;

use poem::{get, IntoEndpoint, Route};

mod index;

pub(crate) async fn get_route() -> Result<impl IntoEndpoint> {
    Ok(Route::new()
        .at("/", get(index::index))
        .at("/2", get(index::index2))
        .at("/3", get(index::index3)))
}
