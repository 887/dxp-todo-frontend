//#[no_mangle] is unsafe, but needed for hot reload.
//https://github.com/rust-lang/rust/issues/111967
#![allow(unsafe_code)]

use no_mangle_if_debug::no_mangle_if_debug;

use crate::Result;

#[no_mangle_if_debug]
pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    Ok(dotenvy::dotenv().map_err(|_| "could not load .env")?)
}

#[no_mangle_if_debug]
pub async fn post_server_data(data: String) -> Result<()> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[no_mangle_if_debug]
pub async fn get_server_data() -> Result<String> {
    Ok("Helloasasd from the server!".to_string())
}
