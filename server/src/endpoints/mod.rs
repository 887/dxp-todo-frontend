use crate::state::State;
use poem::{get, EndpointExt, Route};
use tracing::info;

mod index;

use crate::templates;

pub fn get_route() -> Route {
    let templates = templates::get_templates();
    #[cfg(feature = "hot-reload")]
    templates::watch_directory(templates::TEMPLATE_DIR, &templates);

    let index = Route::new().at("/", get(index::index));

    let state = State {
        templates: &templates,
    };

    let index_with_state = index.data(state);

    let route = Route::new().nest("/", index_with_state); //routers need to be nested

    route
}
