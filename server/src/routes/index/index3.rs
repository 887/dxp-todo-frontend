use crate::{error::CtxtErrExt, state};
use anyhow::Context;
use dxp_code_loc::code_loc;
use poem::i18n::{I18NArgs, I18NBundle};
use poem::{
    handler,
    session::Session,
    web::{Data, Html},
    IntoResponse,
};

#[handler]
pub fn index3(session: &Session, state: Data<&state::State>) -> poem::Result<impl IntoResponse> {
    let locale = crate::session::get_user_language_bundle(&state, session);

    let texts = get_texts(&locale, "name")
        .context(code_loc!())
        .map_ctxt_err()?;

    let err_texts = get_errors(&locale).context(code_loc!()).map_ctxt_err()?;

    let templates = state.get_templates();
    let template = templates
        .get_template("routes/index/index3.jinja")
        .context(code_loc!())
        .map_ctxt_err()?;
    // .map_err(ContextualError::from)?;

    let ctx = minijinja::context! {
        ..texts,
        ..err_texts
    };

    let body = template.render(&ctx).context(code_loc!()).map_ctxt_err()?;
    // .map_err(ContextualError::from)?;

    Ok(Html(body).into_response())
}

pub fn get_errors(locale: &I18NBundle) -> Result<minijinja::Value, anyhow::Error> {
    Ok(minijinja::context! {
        name_missing => locale.text("name_missing").context(code_loc!())?,
    })
}

pub fn get_texts(locale: &I18NBundle, name: &str) -> Result<minijinja::Value, anyhow::Error> {
    Ok(minijinja::context! {
        hello_world => locale.text("hello_world").context(code_loc!())?,
        welcome  => locale.text_with_args("welcome", I18NArgs::default().set("name", name))
        .context(code_loc!())?,
    })
}
