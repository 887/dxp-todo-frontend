//#[no_mangle] is unsafe, but needed for hot reload.
//https://github.com/rust-lang/rust/issues/111967
#![allow(unsafe_code)]

use crate::{cold, Result};

#[no_mangle]
pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    cold::load_env()
}

#[no_mangle]
pub async fn post_server_data(data: String) -> Result<()> {
    cold::post_server_data(data).await
}

#[no_mangle]
pub async fn get_server_data() -> Result<String> {
    cold::get_server_data().await
}
