use anyhow::Result;
use arc_swap::ArcSwap;
use poem::i18n::I18NResources;
use std::sync::{Arc, OnceLock};

use crate::endpoints::i18n::{self, I18NResourcesType};

//https://stackoverflow.com/questions/27221504/how-can-you-make-a-safe-static-singleton-in-rust
static I18N_DATA: OnceLock<I18NResourcesType> = OnceLock::new();

pub fn get() -> Result<&'static I18NResourcesType> {
    let i18n_data = get_from_env()?;
    Ok(&I18N_DATA.get_or_init(|| ArcSwap::new(Arc::new(i18n_data))))
}

#[cfg(feature = "hot-reload")]
fn get_from_env() -> Result<I18NResources> {
    Ok(i18n::get_i18n_data()?)
}

#[cfg(not(feature = "hot-reload"))]
fn get_from_env() -> Result<I18NResources> {
    Ok(i18n::get_i18n_data_from_embedded()?)
}
