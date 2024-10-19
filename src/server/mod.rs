#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

#[cfg(debug_assertions)]
mod hot;
#[cfg(debug_assertions)]
pub use hot::*;

#[cfg(debug_assertions)]
mod cold;
#[cfg(debug_assertions)]
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
