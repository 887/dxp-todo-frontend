//TODO

pub static SESSION_USER_LANGUAGE_NAME: &str = "user_lang"; //i18n

pub fn session_get_user_language(session: &Session) -> Option<String> {
    session.get(SESSION_USER_LANGUAGE_NAME)
}

pub fn session_set_user_language(session: &Session, value: &str) {
    session.set(SESSION_USER_LANGUAGE_NAME, value)
}

pub fn session_get_user_language_bundle(
    state: &Data<&state::FrontendState>,
    session: &Session,
) -> I18NBundle {
    let lang_setting =
        session_get_user_language(session).unwrap_or(state.default_language.to_string());
    let lang_id = LanguageIdentifier::from_str(&lang_setting).unwrap_or(langid!("en-US"));
    let lang = &[lang_id];
    let i18n_data = state.get_i18n_data();
    i18n_data.negotiate_languages(lang)
}
