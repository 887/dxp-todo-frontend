use anyhow::Result;
use std::sync::OnceLock;

use dioxus::prelude::*;
// use dioxus_sdk::i18n::*;
// use dioxus_sdk::translate;

// https://github.com/dioxus-community/dioxus-i18n

// fn app() -> Element {
//     let i18 = use_init_i18n(|| {
//         I18nConfig::new(langid!("en-US"))
//             .with_locale(Locale::new_static(
//                 // Embed
//                 langid!("en-US"),
//                 include_str!("./en-US.ftl"),
//             ))
//             .with_locale(Locale::new_dynamic(
//                 // Load at launch
//                 langid!("es-ES"),
//                 include_str!("./es-ES.ftl"),
//             ))
//     });

//     rsx!(
//         label { { t!("hello", name: "World") } }
//     )
// }

#[cfg(feature = "hot-reload")]
mod watcher;
#[cfg(feature = "hot-reload")]
pub use watcher::*;

mod initializer;
pub use initializer::*;

// pub static VALID_LANGUAGES: &[&str] = &["en-US", "de-DE"];

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
