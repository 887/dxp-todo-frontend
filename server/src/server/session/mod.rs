pub mod api_database_pool;
// mod language;
// pub use language::*;

use anyhow::Result;

use api_database_pool::ApiDatabasePool;

pub async fn get_api_storage(api: String) -> Result<ApiDatabasePool> {
    let storage = api_database_pool::ApiDatabasePool::new(api);
    Ok(storage)
}
