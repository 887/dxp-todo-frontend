use anyhow::Result;
use std::sync::OnceLock;

use crate::endpoints::i18n::{self, I18NResourcesType};

//https://stackoverflow.com/questions/27221504/how-can-you-make-a-safe-static-singleton-in-rust
static I18N_DATA: OnceLock<I18NResourcesType> = OnceLock::new();

#[cfg(feature = "hot-reload")]
pub fn get() -> Result<&'static I18NResourcesType> {
    use arc_swap::ArcSwap;
    use std::sync::Arc;

    let i18n_data = i18n::get_i18n_data()?;
    Ok(&I18N_DATA.get_or_init(|| ArcSwap::new(Arc::new(i18n_data))))
}

#[cfg(not(feature = "hot-reload"))]
pub fn get() -> Result<&'static I18NResourcesType> {
    let i18n_data = i18n::get_i18n_data_from_embedded()?;
    Ok(&I18N_DATA.get_or_init(|| i18n_data))
}
