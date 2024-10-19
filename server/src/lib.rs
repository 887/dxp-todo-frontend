#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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

pub async fn call_backend_with_server() -> Result<i64> {
    tracing::info!("Calling backend");

    use backend::ClientSessionExt;

    let api = "http://127.0.0.1:8000";
    let client = backend::Client::new(&api);
    let response = client.count().table_name("sessions").send().await?;
    Ok(response.count)
}
