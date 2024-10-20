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

mod app;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");

    #[cfg(feature = "path-info")]
    path_info::print_paths();

    #[cfg(feature = "server")]
    dotenvy::dotenv()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "could not load .env"))?;

    #[cfg(any(feature = "server", feature = "web", feature = "desktop"))]
    init();

    // #[cfg(feature = "web")]
    launch(App);

    // //https://github.com/DioxusLabs/dioxus/issues/2380
    // dioxus 0.5.0
    // let cfg = server_only!(
    //     dioxus::fullstack::Config::new().addr(std::net::SocketAddr::from(([0, 0, 0, 0], 3000)))
    // );

    // LaunchBuilder::server().with_cfg(cfg).launch(App);

    // dioxus 0.6.0
    // #[cfg(feature = "server")]
    // {
    //     tokio::runtime::Runtime::new()
    //         .unwrap()
    //         .block_on(async move {
    //             let listener = tokio::net::TcpListener::bind("127.0.0.01:3000")
    //                 .await
    //                 .unwrap();
    //             axum::serve(
    //                 listener,
    //                 axum::Router::new()
    //                     // Server side render the application, serve static assets, and register server functions
    //                     .serve_dioxus_application(ServeConfigBuilder::default(), App)
    //                     .into_make_service(),
    //             )
    //             .await
    //             .unwrap();
    //         });
    // }

    #[cfg(feature = "web")]
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
