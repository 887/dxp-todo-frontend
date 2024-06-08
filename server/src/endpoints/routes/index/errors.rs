#[derive(Serialize)]
struct TranslatedErrs {
    pub name_missing: String,
}

impl TranslatedErrs {
    fn get_text(locale: &I18NBundle) -> Result<TranslatedErrs, I18NError> {
        Ok(TranslatedErrs {
            name_missing: locale.text("name_missing")?,
        })
    }
}
