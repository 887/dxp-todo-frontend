use anyhow::{Ok, Result};
use axum::Router;
use tracing::{info, trace};

// mod index;
// mod r#static;
// mod test;

pub(crate) async fn get_route() -> Result<Router> {
    let route = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                trace!("hello world");
                "Hello, World!"
            }),
        )
        .route(
            "/2",
            axum::routing::get(|| async {
                trace!("hello world 2");
                "Hello, World2!"
            }),
        );

    //session error returns internal server error, we should probs log this
    let app_session = axum::Router::new().route(
        "/",
        axum::routing::get(|| async {
            info!("session route");
            "Hello, Session!"
        }),
    );

    let route = route.nest("/session", app_session);

    // let route = Route::new().at("/", get(index::index));

    // let route = route.nest("/static", r#static::get_route());

    // Ok(route.nest("/test", test::get_route().await?))

    Ok(route)
}
