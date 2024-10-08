use backend::ClientSessionExt;

use axum::async_trait;
use axum_session::DatabasePool;
use dxp_code_loc::code_loc;
use tracing::error;

#[derive(Debug, Clone)]
pub struct ApiDatabasePool {
    client: backend::Client,
}

impl ApiDatabasePool {
    /// Create a new ApiSessionStorage.
    pub fn new(api: String) -> ApiDatabasePool {
        let client = backend::Client::new(&api);
        ApiDatabasePool { client }
    }
}

#[async_trait]
impl DatabasePool for ApiDatabasePool {
    #[inline(always)]
    async fn initiate(&self, _table_name: &str) -> Result<(), axum_session::DatabaseError> {
        Ok(())
    }

    #[inline(always)]
    async fn delete_by_expiry(
        &self,
        table_name: &str,
    ) -> Result<Vec<String>, axum_session::DatabaseError> {
        self.client
            .delete_by_expiry()
            .table_name(table_name)
            .send()
            .await
            .map(|res| res.into_inner())
            .map_err(|err| {
                error!("{}: {}", code_loc!(), err);
                axum_session::DatabaseError::GenericDeleteError(err.to_string())
            })
    }

    #[inline(always)]
    async fn count(&self, table_name: &str) -> Result<i64, axum_session::DatabaseError> {
        self.client
            .count()
            .table_name(table_name)
            .send()
            .await
            .map(|res| res.into_inner().count)
            .map_err(|err| {
                error!("{}: {}", code_loc!(), err);
                axum_session::DatabaseError::GenericSelectError(err.to_string())
            })
    }

    #[inline(always)]
    async fn store(
        &self,
        id: &str,
        session: &str,
        expires: i64,
        table_name: &str,
    ) -> Result<(), axum_session::DatabaseError> {
        self.client
            .store()
            .id(id)
            .session(session)
            .expires(expires)
            .table_name(table_name)
            .send()
            .await
            .map(|res| res.into_inner())
            .map_err(|err| {
                error!("{}: {}", code_loc!(), err);
                axum_session::DatabaseError::GenericInsertError(err.to_string())
            })
    }

    #[inline(always)]
    async fn load(
        &self,
        id: &str,
        table_name: &str,
    ) -> Result<Option<String>, axum_session::DatabaseError> {
        self.client
            .load()
            .id(id)
            .table_name(table_name)
            .send()
            .await
            .map(|res| res.into_inner().value)
            .map_err(|err| {
                error!("{}: {}", code_loc!(), err);
                axum_session::DatabaseError::GenericSelectError(err.to_string())
            })
    }

    #[inline(always)]
    async fn delete_one_by_id(
        &self,
        id: &str,
        table_name: &str,
    ) -> Result<(), axum_session::DatabaseError> {
        self.client
            .delete_one_by_id()
            .id(id)
            .table_name(table_name)
            .send()
            .await
            .map(|res| res.into_inner())
            .map_err(|err| {
                error!("{}: {}", code_loc!(), err);
                axum_session::DatabaseError::GenericDeleteError(err.to_string())
            })
    }

    #[inline(always)]
    async fn exists(
        &self,
        id: &str,
        table_name: &str,
    ) -> Result<bool, axum_session::DatabaseError> {
        self.client
            .exists()
            .id(id)
            .table_name(table_name)
            .send()
            .await
            .map(|res| res.into_inner().value)
            .map_err(|err| {
                error!("{}: {}", code_loc!(), err);
                axum_session::DatabaseError::GenericSelectError(err.to_string())
            })
    }

    #[inline(always)]
    async fn delete_all(&self, table_name: &str) -> Result<(), axum_session::DatabaseError> {
        self.client
            .delete_all()
            .table_name(table_name)
            .send()
            .await
            .map(|res| res.into_inner())
            .map_err(|err| {
                error!("{}: {}", code_loc!(), err);
                axum_session::DatabaseError::GenericDeleteError(err.to_string())
            })
    }

    #[inline(always)]
    async fn get_ids(&self, table_name: &str) -> Result<Vec<String>, axum_session::DatabaseError> {
        self.client
            .get_ids()
            .table_name(table_name)
            .send()
            .await
            .map(|res| res.into_inner())
            .map_err(|err| {
                error!("{}: {}", code_loc!(), err);
                axum_session::DatabaseError::GenericSelectError(err.to_string())
            })
    }

    #[inline(always)]
    fn auto_handles_expiry(&self) -> bool {
        false
    }
}

// impl SessionStorage for ApiSessionDatabasePool {
//     async fn load_session<'a>(
//         &'a self,
//         session_id: &'a str,
//     ) -> Result<Option<BTreeMap<String, Value>>> {
//         let res = self.client.load_session(session_id).await;

//         match res {
//             Ok(res) => {
//                 let inner = res.into_inner();
//                 if let Some(inner) = inner {
//                     let map: BTreeMap<String, Value> = serde_json::from_str(&inner)
//                         .map_err(|err| map_backend_err(code_loc!(), err.into()))?;
//                     Ok(Some(map))
//                 } else {
//                     Ok(None)
//                 }
//             }
//             Err(err) => {
//                 if err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
//                     Ok(None)
//                 } else {
//                     Err(map_backend_err(code_loc!(), err.into()))
//                 }
//             }
//         }
//     }

//     async fn update_session<'a>(
//         &'a self,
//         session_id: &'a str,
//         entries: &'a BTreeMap<String, Value>,
//         expires: Option<Duration>,
//     ) -> Result<()> {
//         let body = backend::types::UpdateSessionValue {
//             entries: serde_json::to_string(entries)
//                 .map_err(|err| map_backend_err(code_loc!(), err.into()))?,
//             expires: expires.map(|t| Utc::now().timestamp() + t.as_secs() as i64),
//         };

//         let res = self
//             .client
//             .update_session(session_id, &body)
//             .await
//             .map_err(|err| map_backend_err(code_loc!(), err.into()))?;

//         if res.status() == 200 {
//             Ok(())
//         } else {
//             client_error(res, "Server did not update_session and return 200")
//         }
//     }

//     async fn remove_session<'a>(&'a self, session_id: &'a str) -> Result<()> {
//         let res = self
//             .client
//             .remove_session(session_id)
//             .await
//             .map_err(|err| map_backend_err(code_loc!(), err.into()))?;

//         if res.status() == 200 {
//             Ok(())
//         } else {
//             client_error(res, "Server did not remove_session and return 200")
//         }
//     }
// }

// fn client_error<T, V>(
//     res: backend::ResponseValue<T>,
//     err_msg: &str,
// ) -> std::result::Result<V, poem::Error> {
//     let status_code = match StatusCode::from_u16(res.status().as_u16()) {
//         Ok(s) => s,
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
//     };

//     Err(poem::error::Error::from_string(err_msg, status_code))
// }

// fn map_backend_err(code_loc: String, err: anyhow::Error) -> poem::Error {
//     error!("{:?}\n{}", err, code_loc);

//     poem::error::Error::from_string(
//         format!("{:?}\n{}", err, code_loc),
//         StatusCode::INTERNAL_SERVER_ERROR,
//     )
// }
