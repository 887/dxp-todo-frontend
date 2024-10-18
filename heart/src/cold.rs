use crate::Result;

pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    Ok(dotenvy::dotenv().map_err(|_| "could not load .env")?)
}

pub async fn post_server_data(data: String) -> Result<()> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

pub async fn get_server_data() -> Result<String> {
    Ok("Hello from the server!".to_string())
}
