use std::{collections::BTreeMap, time::Duration};

use dxp_code_loc::code_loc;
use poem::{http::StatusCode, session::SessionStorage, Result};
use serde_json::Value;
use tracing::error;

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
        let res = self.client.load_session(session_id).await;

        if let Ok(res) = res {
            let inner = res.into_inner();
            let map = BTreeMap::from_iter(inner.iter().map(|(i, e)| (i.to_string(), e.clone())));
            return Ok(Some(map));
        } else if let Err(backend::Error::ErrorResponse(ref err)) = res {
            if err.status() == 404 {
                return Ok(None);
            }
        }

        let res = res.map_err(|err| map_backend_err(code_loc!(), err))?;

        client_error(res, "Server did not load_session and return 200")
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
            expires: expires.map(|t| t.as_secs()),
        };

        let res = self
            .client
            .update_session(session_id, &body)
            .await
            .map_err(|err| map_backend_err(code_loc!(), err))?;

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
            .map_err(|err| map_backend_err(code_loc!(), err))?;

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

fn map_backend_err(code_loc: String, err: backend::Error) -> poem::Error {
    error!("{:?}\n{}", err, code_loc);

    poem::error::Error::new(err, StatusCode::INTERNAL_SERVER_ERROR)
}
