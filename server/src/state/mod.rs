mod default_language;

use anyhow::Result;

use derivative::Derivative;

#[cfg(feature = "hot-reload")]
use std::sync::Arc;
#[cfg(feature = "hot-reload")]
mod watcher;

use super::{
    i18n,
    i18n::I18NResourcesType,
    templates::{self, TemplatesType},
};

use super::css;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct State {
    pub templates: &'static TemplatesType,
    pub default_language: &'static str,
    #[derivative(Debug = "ignore")]
    pub i18n_data: &'static I18NResourcesType,
}

impl State {
    pub async fn new() -> Result<State> {
        let templates = templates::get_templates()?;
        let default_language = default_language::get()?;
        let i18n_data = i18n::get()?;

        Ok(State {
            templates,
            default_language,
            i18n_data,
        })
    }

    #[cfg(feature = "hot-reload")]
    pub fn watch(&self) {
        let callback_templates =
            watcher::CallbackOneParam::wrap(&templates::handle_event, self.templates);
        watcher::watch_directory(templates::TEMPLATE_DIR, callback_templates);

        let callback_i18n = watcher::CallbackOneParam::wrap(&i18n::handle_event, self.i18n_data);
        watcher::watch_directory(i18n::I18N_DIR, callback_i18n);

        let callback_css = watcher::CallbackNoParams::wrap(&css::handle_event);
        watcher::watch_directory(css::STYLE_DIR, callback_css);
    }

    #[cfg(not(feature = "hot-reload"))]
    pub fn get_templates(&self) -> &'static Minijinja<'static> {
        self.templates
    }
    #[cfg(feature = "hot-reload")]
    pub fn get_templates(&self) -> arc_swap::Guard<Arc<Minijinja<'static>>> {
        self.templates.load()
    }

    #[cfg(not(feature = "hot-reload"))]
    pub fn get_i18n_data(&self) -> &'static I18NResources {
        self.i18n_data
    }

    #[cfg(feature = "hot-reload")]
    pub fn get_i18n_data(&self) -> arc_swap::Guard<Arc<I18NResources>> {
        self.i18n_data.load()
    }
}
