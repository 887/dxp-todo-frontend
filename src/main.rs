#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

#[cfg(not(feature = "web"))]
mod server;

#[cfg(feature = "web")]
mod web;

#[allow(dead_code)]
pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> std::io::Result<()> {
    #[cfg(not(feature = "web"))]
    {
        server::main()
    }

    #[cfg(feature = "web")]
    {
        web::main()
    }
}
