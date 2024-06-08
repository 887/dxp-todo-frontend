use std::{collections::BTreeMap, time::Duration};

use poem::{http::StatusCode, session::SessionStorage, Result};
use serde_json::Value;

use backend;

#[derive(Clone)]
pub struct ApiSessionStorage {
    client: backend::Client,
}

impl ApiSessionStorage {
    /// Create a new ApiSessionStorage.
    pub fn new(api: String) -> ApiSessionStorage {
        let client = backend::Client::new(&api);
        ApiSessionStorage { client }
    }
}

impl SessionStorage for ApiSessionStorage {
    async fn load_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> Result<Option<BTreeMap<String, Value>>> {
        let res = self
            .client
            .load_session(session_id)
            .await
            .map_err(map_backend_err)?;

        if res.status() == 200 {
            let inner = res.into_inner();
            if (!inner.exists) {
                return Ok(None);
            }
            let map = BTreeMap::from_iter(
                inner
                    .entries
                    .iter()
                    .map(|(i, e)| (i.to_string(), e.clone())),
            );
            Ok(Some(map))
        } else {
            client_error(res, "Server did not load_session and return 200")
        }
    }

    async fn update_session<'a>(
        &'a self,
        session_id: &'a str,
        entries: &'a BTreeMap<String, Value>,
        expires: Option<Duration>,
    ) -> Result<()> {
        let body = backend::types::UpdateSessionValue {
            entries: entries
                .iter()
                .map(|(i, e)| (i.to_string(), e.clone()))
                .collect::<serde_json::Map<String, Value>>(),
            expires: expires.map(|t| t.as_secs() as u64),
        };

        let res = self
            .client
            .update_session(session_id, &body)
            .await
            .map_err(map_backend_err)?;

        if res.status() == 200 {
            Ok(())
        } else {
            client_error(res, "Server did not update_session and return 200")
        }
    }

    async fn remove_session<'a>(&'a self, session_id: &'a str) -> Result<()> {
        let res = self
            .client
            .remove_session(session_id)
            .await
            .map_err(map_backend_err)?;

        if res.status() == 200 {
            Ok(())
        } else {
            client_error(res, "Server did not remove_session and return 200")
        }
    }
}

fn client_error<T, V>(
    res: backend::ResponseValue<T>,
    err_msg: &str,
) -> std::result::Result<V, poem::Error> {
    let status_code = match StatusCode::from_u16(res.status().as_u16()) {
        Ok(s) => s,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    Err(poem::error::Error::from_string(err_msg, status_code))
}

fn map_backend_err(err: backend::Error) -> poem::Error {
    poem::error::Error::new(err, StatusCode::INTERNAL_SERVER_ERROR)
}
