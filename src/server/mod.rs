#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

#[cfg(feature = "server")]
pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(feature = "path-info")]
mod path_info;

#[cfg(all(feature = "server", feature = "hot-reload"))]
mod hot;
#[cfg(all(feature = "server", feature = "hot-reload"))]
pub use hot::*;

#[cfg(all(feature = "server", not(feature = "hot-reload")))]
mod cold;
#[cfg(all(feature = "server", not(feature = "hot-reload")))]
pub use cold::*;

#[cfg(feature = "log")]
pub fn get_log_subscription() -> std::io::Result<dxp_logging::LogGuard> {
    dxp_logging::subscribe_thread_with_default().map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("could not get log subscription: {:?}", err),
        )
    })
}
