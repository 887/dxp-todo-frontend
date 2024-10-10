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

#[cfg(feature = "host")]
mod server;

#[cfg(feature = "web")]
pub mod web;

#[cfg(all(feature = "host", not(feature = "hot-reload")))]
mod cold;
#[cfg(all(feature = "host", not(feature = "hot-reload")))]
pub use cold::*;
#[cfg(all(feature = "host", feature = "hot-reload"))]
mod hot;
#[cfg(all(feature = "host", feature = "hot-reload"))]
pub use hot::*;
