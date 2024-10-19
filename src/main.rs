#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
#![allow(non_snake_case)]

use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[cfg(feature = "path-info")]
mod path_info;

#[cfg(feature = "server")]
mod server;

mod app;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");

    #[cfg(feature = "server")]
    dotenvy::dotenv()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "could not load .env"))?;

    #[cfg(all(debug_assertions, feature = "server", feature = "log"))]
    crate::server::log_reload();

    #[cfg(any(feature = "server", feature = "web", feature = "desktop"))]
    init();

    launch(App);

    Ok(())
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
