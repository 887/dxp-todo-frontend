use anyhow::Context;
use dxp_code_loc::code_loc;
use minijinja::context;
use poem::i18n::{I18NArgs, I18NBundle};

pub fn get(locale: &I18NBundle, name: &str) -> Result<minijinja::Value, anyhow::Error> {
    Ok(context! {
        hello_world => locale.text("hello_world").context(code_loc!())?,
        welcome  => locale.text_with_args("welcome", I18NArgs::default().set("name", name))
        .context(code_loc!())?,
    })
}
