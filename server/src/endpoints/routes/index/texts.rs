use poem::{error::I18NError, i18n::I18NBundle};
use serde::Serialize;

#[derive(Serialize)]
struct TranslatedTexts {
    pub hello: String,
    pub welcome: String,
}

impl TranslatedTexts {
    fn get_text(locale: &I18NBundle) -> Result<TranslatedTexts, I18NError> {
        Ok(TranslatedTexts {
            hello: locale.text("user_ssh_keys")?,
            welcome: locale.text("logout")?,
        })
    }
}
