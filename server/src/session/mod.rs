mod api_database_pool;
// mod language;
// pub use language::*;

use std::env;

use anyhow::Context;
use anyhow::Result;
// use poem::{
//     session::{CookieConfig, ServerSession, SessionStorage},
//     web::cookie::CookieKey,
// };

use axum_session::DatabasePool;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

// pub fn get_session_middleware<S>(storage: S) -> Result<ServerSession<S>>
// where
//     S: SessionStorage,
// {
//     let cookie_key = env::var("COOKIE_KEY").context("COOKIE_KEY is not set")?;

//     let cookie_key_bytes =
//         engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
//             .decode(cookie_key)
//             .context("COOKIE_KEY not base64")?;

//     let cookie_key = CookieKey::from(&cookie_key_bytes);

//     Ok(ServerSession::new(
//         CookieConfig::signed(cookie_key),
//         storage,
//     ))
// }

pub async fn get_api_storage(api: String) -> Result<impl DatabasePool> {
    let storage = api_database_pool::ApiDatabasePool::new(api);
    Ok(storage)
}

#[cfg(feature = "redis")]
pub async fn get_redis_storage(db: DatabaseConnection) -> Result<impl SessionStorage> {
    let redis_url = env::var("REDIS_URL").context("REDIS_URL is not set")?;
    let client = redis::Client::open(redis_url)?;
    let con_manager = redis::aio::ConnectionManager::new(client).await?;
    poem::session::RedisStorage::new(con_manager)
}
