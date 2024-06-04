use std::{collections::BTreeMap, ops::Add, time::Duration};

use chrono::Utc;
use poem::{error::InternalServerError, session::SessionStorage, Result};
use serde_json::Value;

#[derive(Clone)]
pub struct ApiSessionStorage {
    api: String,
}

impl ApiSessionStorage {
    /// Create an [`PgSessionStorage`].
    pub fn new(api: String) -> ApiSessionStorage {
        ApiSessionStorage { api }
    }
}

impl SessionStorage for ApiSessionStorage {
    async fn load_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> Result<Option<BTreeMap<String, Value>>> {
        // const LOAD_SESSION_SQL: &str = r#"
        //     select session from {table_name}
        //         where id = $1 and (expires is null or expires > $2)
        //     "#;

        // let maybe_model = poem_sessions::Entity::find()
        //     .filter(poem_sessions::Column::Id.eq(session_id))
        //     .filter(
        //         poem_sessions::Column::Expires
        //             .is_null()
        //             .or(poem_sessions::Column::Expires.gt(Utc::now())),
        //     )
        //     .one(&self.db)
        //     .await
        //     .map_err(InternalServerError)?;

        // if let Some(model) = maybe_model {
        //     let res: serde_json::Result<BTreeMap<String, Value>> =
        //         serde_json::from_value(model.session);
        //     match res {
        //         Ok(btr_map) => Ok(Some(btr_map)),
        //         Err(_err) => Ok(None),
        //     }
        // } else {
        Ok(None)
        // }
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
