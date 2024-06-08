use std::{collections::BTreeMap, time::Duration};

use poem::{http::StatusCode, session::SessionStorage, Result};
use serde_json::Value;

use backend;

#[derive(Clone)]
pub struct ApiSessionStorage {
    client: backend::Client,
}

impl ApiSessionStorage {
    /// Create an [`PgSessionStorage`].
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
        let res = match self.client.load_session(session_id).await {
            Ok(r) => r,
            Err(err) => {
                return Err(poem::error::Error::new(
                    err,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        };
        if res.status() == 200 {
            let inner = res.into_inner();
            let map = BTreeMap::from_iter(inner.iter().map(|(i, e)| (i.to_string(), e.clone())));
            return Ok(Some(map));
        }

        Ok(None)
    }

    async fn update_session<'a>(
        &'a self,
        session_id: &'a str,
        entries: &'a BTreeMap<String, Value>,
        expires: Option<Duration>,
    ) -> Result<()> {
        // const UPDATE_SESSION_SQL: &str = r#"
        //     insert into {table_name} (id, session, expires) values ($1, $2, $3)
        //         on conflict(id) do update set
        //             expires = excluded.expires,
        //             session = excluded.session
        // "#;

        //https://www.sea-ql.org/SeaORM/docs/basic-crud/update/
        //https://www.sea-ql.org/SeaORM/docs/basic-crud/insert/

        // let expires = match expires {
        //     Some(expires) => {
        //         Some(chrono::Duration::from_std(expires).map_err(InternalServerError)?)
        //     }
        //     None => None,
        // };

        // let session_map = serde_json::Map::from_iter(entries.clone());

        // let model = poem_sessions::ActiveModel {
        //     id: ActiveValue::set(session_id.to_owned()),
        //     session: ActiveValue::set(sea_orm::JsonValue::from(session_map)),
        //     expires: ActiveValue::set(expires.map(|expires| Utc::now().add(expires))),
        // };

        // poem_sessions::Entity::insert(model.clone())
        //     .on_conflict(
        //         sea_query::OnConflict::column(poem_sessions::Column::Id)
        //             .update_columns([
        //                 poem_sessions::Column::Expires,
        //                 poem_sessions::Column::Session,
        //             ])
        //             .to_owned(),
        //     )
        //     .exec(&self.db)
        //     .await
        //     .map_err(InternalServerError)?;

        Ok(())
    }

    async fn remove_session<'a>(&'a self, session_id: &'a str) -> Result<()> {
        // const REMOVE_SESSION_SQL: &str = r#"
        //     delete from {table_name} where id = $1
        // "#;

        // poem_sessions::Entity::delete_many()
        //     .filter(poem_sessions::Column::Id.eq(session_id))
        //     .exec(&self.db)
        //     .await
        //     .map_err(InternalServerError)?;

        Ok(())
    }
}
