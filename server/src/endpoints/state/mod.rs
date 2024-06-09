mod default_language;
mod i18n;

use anyhow::Result;

use derivative::Derivative;
use minijinja::Environment as Minijinja;
use poem::i18n::I18NResources;
#[cfg(feature = "hot-reload")]
use std::sync::Arc;

use super::{
    i18n::I18NResourcesType,
    templates::{self, TemplatesType},
};

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct State {
    pub templates: &'static TemplatesType,
    pub default_language: &'static str,
    #[derivative(Debug = "ignore")]
    pub i18n_data: &'static I18NResourcesType,
}

impl State {
    pub fn new() -> Result<State> {
        let templates = templates::get_templates();
        let default_language = default_language::get()?;
        let i18n_data = i18n::get()?;

        Ok(State {
            templates,
            default_language,
            i18n_data,
        })
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
