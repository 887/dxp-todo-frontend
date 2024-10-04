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

fn main() {
    #[cfg(not(feature = "web"))]
    server::main();

    #[cfg(feature = "web")]
    web::main();
}
