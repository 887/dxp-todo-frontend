use crate::state::State;
use error::ErrorMiddleware;
use poem::{get, Endpoint, EndpointExt, Route};

mod error;
mod index;

use crate::templates;

pub fn get_route() -> impl Endpoint {
    let templates = templates::get_templates();
    #[cfg(feature = "hot-reload")]
    templates::watch_directory(templates::TEMPLATE_DIR, &templates);

    let index = Route::new().at("/", get(index::index));

    let state = State {
        templates: &templates,
    };

    let index_with_state = index.data(state);

    let route = Route::new().nest("/", index_with_state); //routers need to be nested

    let error_middleware = ErrorMiddleware {
        templates: &templates,
    };
    route.with(error_middleware)
}
