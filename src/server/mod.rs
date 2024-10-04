#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

#[cfg(feature = "path-info")]
mod path_info;

#[cfg(feature = "hot-reload")]
pub mod hot;
#[cfg(feature = "hot-reload")]
mod observe;

#[cfg(not(feature = "hot-reload"))]
pub mod cold;

#[cfg(feature = "log")]
pub fn get_log_subscription() -> std::io::Result<dxp_logging::LogGuard> {
    dxp_logging::get_subscription().map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("could not get log subscription: {:?}", err),
        )
    })
}
