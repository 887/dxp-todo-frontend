#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// mod css;
// mod endpoint;
// mod error;
// mod i18n;
// mod routes;
// mod state;
// mod templates;

#[cfg(not(feature = "web"))]
mod server;

#[cfg(feature = "web")]
pub mod web;

#[cfg(all(not(feature = "web"), not(feature = "hot-reload")))]
mod cold;
#[cfg(all(not(feature = "web"), not(feature = "hot-reload")))]
pub use cold::*;
#[cfg(all(not(feature = "web"), feature = "hot-reload"))]
mod hot;
#[cfg(all(not(feature = "web"), feature = "hot-reload"))]
pub use hot::*;
