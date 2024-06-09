use anyhow::Context;
use dxp_code_loc::code_loc;
use poem::{
    handler,
    session::Session,
    web::{Data, Html},
    IntoResponse,
};
use texts::TranslatedTexts;
use tracing::trace;

use crate::endpoints::{
    error::ContextualError, session::language::get_user_language_bundle, state,
};

// mod errors;
mod texts;

pub static SESSION_INDEX_COUNTER: &str = "index_counter";

#[handler]
pub fn index(session: &Session) -> String {
    let counter = session
        .get::<usize>(SESSION_INDEX_COUNTER)
        .map_or(0, |v| v + 1);

    session.set(SESSION_INDEX_COUNTER, counter);

    let hello = format!("hello world! {}", counter);
    trace!("{}", &hello);

    hello.to_owned()
}

#[handler]
pub fn index2() -> String {
    let hello = "hello world!".to_string();
    trace!("{}", &hello);

    hello.to_owned()
}

#[handler]
pub fn index3(session: &Session, state: Data<&state::State>) -> poem::Result<impl IntoResponse> {
    let locale = get_user_language_bundle(&state, session);
    // let account_maybe = session_get_active_account(session);
    let texts = TranslatedTexts::get_text(&locale)
        .context(code_loc!())
        .map_err(ContextualError::from)?;

    let templates = state.get_templates();
    let template = templates
        .get_template("routes/index/index.jinja")
        .context(code_loc!())
        .map_err(ContextualError::from)?;

    let ctx = minijinja::context! {
        t => texts,
    };

    let body = template
        .render(&ctx)
        .context(code_loc!())
        .map_err(ContextualError::from)?;

    Ok(Html(body).into_response())
}
