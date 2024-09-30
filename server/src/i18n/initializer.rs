#[cfg(feature = "hot-reload")]
use std::path::Path;

// https://github.com/dioxus-community/dioxus-i18n

use poem::i18n::I18NResources;
#[cfg(not(feature = "hot-reload"))]
use poem::{i18n::unic_langid::LanguageIdentifier, i18n::I18NResourcesBuilder};
#[cfg(not(feature = "hot-reload"))]
use std::str::FromStr;

use anyhow::Result;

#[cfg(feature = "hot-reload")]
pub static I18N_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/i18n/");

//https://docs.rs/include_dir/latest/include_dir/index.html
#[cfg(not(feature = "hot-reload"))]
static I18N_DIR_FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/i18n/");

#[cfg(not(feature = "hot-reload"))]
use include_dir::{include_dir, Dir};

#[cfg(feature = "hot-reload")]
pub fn get_i18n_data() -> Result<I18NResources, poem::error::I18NError> {
    let path_str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/i18n");
    let p = Path::new(path_str);
    I18NResources::builder().add_path(p).build()
}

#[cfg(not(feature = "hot-reload"))]
pub fn get_i18n_data_from_embedded() -> Result<I18NResources> {
    use std::ffi::OsStr;

    use anyhow::Context;
    use dxp_code_loc::code_loc;

    let mut builder = I18NResources::builder();

    let dirs = I18N_DIR_FILES.dirs();
    for dir in dirs.into_iter() {
        let language = match dir
            .path()
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| LanguageIdentifier::from_str(name).ok())
        {
            Some(language_path) => language_path,
            None => continue,
        };

        let files = dir
            .files()
            .into_iter()
            .map(|f| (f.path(), f.contents_utf8()))
            .filter(|(path, _contents)| {
                path.extension()
                    .and_then(OsStr::to_str)
                    .unwrap_or("")
                    .ends_with("ftl")
            })
            .map(|(path, contents)| {
                (
                    path.to_str()
                        .unwrap_or_else(|| panic!("i18n path to_str() failed")),
                    contents.unwrap_or_else(|| panic!("i18n contents not utf8")),
                )
            });

        for (_path, contents) in files {
            builder = load_resources_from_path(builder, &language, contents);
        }
    }

    builder.build().context(code_loc!())
}

#[cfg(not(feature = "hot-reload"))]
fn load_resources_from_path(
    mut builder: I18NResourcesBuilder,
    language: &LanguageIdentifier,
    contents: &str,
) -> I18NResourcesBuilder {
    let filtered_lines = contents
        .lines()
        .filter(|l| !l.trim_start().starts_with("#"));

    for line in filtered_lines {
        builder = builder.add_ftl(language.to_string(), line);
    }
    builder
}
