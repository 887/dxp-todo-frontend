#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "web")]
mod web;

#[cfg(feature = "server")]
#[allow(dead_code)]
pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(all(feature = "server", feature = "hot-reload"))]
fn main() -> std::io::Result<()> {
    server::hot::main()
}

#[cfg(all(feature = "server", not(feature = "hot-reload")))]
fn main() -> std::io::Result<()> {
    server::cold::main()
}

#[cfg(feature = "web")]
fn main() -> std::io::Result<()> {
    web::main()
}
