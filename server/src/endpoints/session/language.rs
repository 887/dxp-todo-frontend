use std::str::FromStr;

use poem::{
    i18n::{
        unic_langid::{langid, LanguageIdentifier},
        I18NBundle,
    },
    session::Session,
    web::Data,
};

use crate::endpoints::state;

static SESSION_USER_LANGUAGE_NAME: &str = "user_lang"; //i18n

pub fn get_user_language(session: &Session) -> Option<String> {
    session.get(SESSION_USER_LANGUAGE_NAME)
}

// pub fn set_user_language(session: &Session, value: &str) {
//     session.set(SESSION_USER_LANGUAGE_NAME, value)
// }

pub fn get_user_language_bundle(state: &Data<&state::State>, session: &Session) -> I18NBundle {
    let lang_setting = get_user_language(session).unwrap_or(state.default_language.to_string());
    let lang_id = LanguageIdentifier::from_str(&lang_setting).unwrap_or(langid!("en-US"));
    let lang = &[lang_id];
    let i18n_data = state.get_i18n_data();
    i18n_data.negotiate_languages(lang)
}
