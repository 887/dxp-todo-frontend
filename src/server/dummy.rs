#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn post_server_data(_data: String) -> Result<()> {
    Err(anyhow::anyhow!("Not implemented").into())
}

pub async fn get_server_data() -> Result<String> {
    Err(anyhow::anyhow!("Not implemented").into())
}
