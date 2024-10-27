#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
#![allow(non_snake_case)]

use app::App;
use dioxus_logger::tracing;

#[allow(unused)]
use dioxus::prelude::*;

#[cfg(feature = "path-info")]
mod path_info;

mod app;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// The entry point for the server
#[cfg(all(feature = "server", feature = "server-axum"))]
#[tokio::main]
async fn main() -> Result<()> {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");

    init();

    // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
    // and we use the generated address the CLI gives us
    let address = dioxus_cli_config::fullstack_address_or_localhost();

    // Set up the axum router
    let router = axum::Router::new()
        // You can add a dioxus application to the router with the `serve_dioxus_application` method
        // This will add a fallback route to the router that will serve your component and server functions
        .serve_dioxus_application(ServeConfigBuilder::default(), App);

    // Finally, we can launch the server
    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    tracing::info!("listening on {}", address);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

#[cfg(any(
    not(feature = "server"),
    all(feature = "server", not(feature = "server-axum"))
))]
fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");

    init();

    dioxus::launch(App);
}

#[cfg(not(any(feature = "server", feature = "web", feature = "desktop")))]
fn init() {
    tracing::info!("starting unknown app");
}

#[cfg(feature = "server")]
fn init() {
    tracing::info!("starting server app");
}

#[cfg(feature = "web")]
fn init() {
    tracing::info!("starting web app");
}

#[cfg(feature = "desktop")]
fn init() {
    tracing::info!("starting desktop app");
}
