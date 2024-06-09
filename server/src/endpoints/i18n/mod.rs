use anyhow::Result;
use poem::i18n::I18NResources;
use std::sync::OnceLock;

#[cfg(feature = "hot-reload")]
mod watcher;
#[cfg(feature = "hot-reload")]
pub use watcher::*;

mod initializer;
pub use initializer::*;

pub static VALID_LANGUAGES: &[&str] = &["en-US", "de-DE"];

#[cfg(feature = "hot-reload")]
pub type I18NResourcesType = arc_swap::ArcSwap<I18NResources>;

#[cfg(not(feature = "hot-reload"))]
pub type I18NResourcesType = I18NResources;

//https://stackoverflow.com/questions/27221504/how-can-you-make-a-safe-static-singleton-in-rust
static I18N_DATA: OnceLock<I18NResourcesType> = OnceLock::new();

#[cfg(feature = "hot-reload")]
pub fn get() -> Result<&'static I18NResourcesType> {
    use arc_swap::ArcSwap;
    use std::sync::Arc;

    let i18n_data = get_i18n_data()?;
    Ok(I18N_DATA.get_or_init(|| ArcSwap::new(Arc::new(i18n_data))))
}

#[cfg(not(feature = "hot-reload"))]
pub fn get() -> Result<&'static I18NResourcesType> {
    let i18n_data = get_i18n_data_from_embedded()?;
    Ok(I18N_DATA.get_or_init(|| i18n_data))
}
