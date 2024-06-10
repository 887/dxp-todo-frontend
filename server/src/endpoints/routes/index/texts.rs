use anyhow::Context;
use dxp_code_loc::code_loc;
use poem::i18n::{I18NArgs, I18NBundle};
use serde::Serialize;

#[derive(Serialize)]
pub struct TranslatedTexts {
    pub hello: String,
    pub welcome: String,
}

impl TranslatedTexts {
    pub fn get_text(locale: &I18NBundle, name: &str) -> Result<TranslatedTexts, anyhow::Error> {
        Ok(TranslatedTexts {
            hello: locale.text("hello_world").context(code_loc!())?,
            welcome: locale
                .text_with_args("welcome", I18NArgs::default().set("name", name))
                .context(code_loc!())?,
        })
    }
}
