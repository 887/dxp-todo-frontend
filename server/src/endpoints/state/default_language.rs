use anyhow::Result;
use std::sync::OnceLock;

//https://stackoverflow.com/questions/27221504/how-can-you-make-a-safe-static-singleton-in-rust
static DEFAULT_LANGUAGE: OnceLock<String> = OnceLock::new();

pub fn get() -> Result<&'static str> {
    let default_language = get_from_env()?;
    Ok(DEFAULT_LANGUAGE.get_or_init(|| default_language))
}

fn get_from_env() -> Result<String> {
    match std::env::var("DEFAULT_LANGUAGE") {
        Ok(val) => Ok(val),
        Err(err) => match err {
            std::env::VarError::NotPresent => Ok("en-US".to_string()),
            std::env::VarError::NotUnicode(_) => {
                Err(anyhow::anyhow!("DATABASE_MAX_CONNECTIONS is not unicode"))
            }
        },
    }
}
