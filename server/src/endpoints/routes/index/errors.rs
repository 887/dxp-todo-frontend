use anyhow::Context;
use dxp_code_loc::code_loc;
use minijinja::context;
use poem::i18n::I18NBundle;

pub fn get(locale: &I18NBundle) -> Result<minijinja::Value, anyhow::Error> {
    Ok(context! {
        name_missing => locale.text("name_missing").context(code_loc!())?,
    })
}
