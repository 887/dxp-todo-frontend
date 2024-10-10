use std::env;

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

pub fn get_session_middleware<S>(storage: S) -> Result<ServerSession<S>>
where
    S: SessionStorage,
{
    let cookie_key = env::var("COOKIE_KEY").context("COOKIE_KEY is not set")?;

    let cookie_key_bytes =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
            .decode(cookie_key)
            .context("COOKIE_KEY not base64")?;

    let cookie_key = CookieKey::from(&cookie_key_bytes);

    Ok(ServerSession::new(
        CookieConfig::signed(cookie_key),
        storage,
    ))
}
