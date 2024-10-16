#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
mod app;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "web")]
mod web;

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
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
