use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::{Deserialize, Serialize};

//https://github.com/dxps/fullstack-rust-axum-dioxus-rwa/tree/main/frontend
//https://docs.rs/dioxus-fullstack/latest/dioxus_fullstack/
//https://crates.io/crates/dioxus-hot-reload

#[cfg(feature = "web")]
fn main() {
    // Hydrate the application on the client
    dioxus_web::launch::launch_cfg(app, dioxus_web::Config::new().hydrate(true));
}
