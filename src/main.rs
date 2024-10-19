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

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");

    #[cfg(all(feature = "hot-reload", feature = "server", feature = "log"))]
    crate::server::log_reload();

    #[cfg(any(feature = "server", feature = "web", feature = "desktop"))]
    init();

    launch(App);
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
