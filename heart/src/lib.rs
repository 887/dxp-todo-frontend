#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// mod i18n;

#[cfg(feature = "server")]
mod routes;

#[cfg(feature = "server")]
mod css;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "web")]
pub mod web;

#[cfg(all(feature = "server", not(feature = "hot-reload")))]
mod cold;
#[cfg(all(feature = "server", not(feature = "hot-reload")))]
pub use cold::*;
#[cfg(all(feature = "server", feature = "hot-reload"))]
mod hot;
#[cfg(all(feature = "server", feature = "hot-reload"))]
pub use hot::*;
