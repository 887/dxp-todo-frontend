use anyhow::Context;
use dxp_code_loc::code_loc;
use poem::i18n::I18NBundle;
use serde::Serialize;

#[derive(Serialize)]
pub struct TranslatedErrs {
    pub name_missing: String,
}

impl TranslatedErrs {
    pub fn get_text(locale: &I18NBundle) -> Result<TranslatedErrs, anyhow::Error> {
        Ok(TranslatedErrs {
            name_missing: locale.text("name_missing").context(code_loc!())?,
        })
    }
}
