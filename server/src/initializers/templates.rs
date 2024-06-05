#![allow(clippy::panic)]

#[cfg(not(feature = "hot-reload"))]
use include_dir::{include_dir, Dir};
#[cfg(feature = "hot-reload")]
use minijinja::path_loader;
use minijinja::Environment as Minijinja;

#[cfg(feature = "hot-reload")]
pub static TEMPLATE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/endpoints/");

#[cfg(feature = "hot-reload")]
pub fn get_templates() -> Minijinja<'static> {
    let mut jinja = Minijinja::new();
    jinja.set_loader(path_loader(TEMPLATE_DIR));
    jinja
}

//https://docs.rs/include_dir/latest/include_dir/index.html
#[cfg(not(feature = "hot-reload"))]
static TEMPLATE_DIR_FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/endpoints/");

#[cfg(not(feature = "hot-reload"))]
pub fn get_templates_embedded() -> Minijinja<'static> {
    let mut jinja = Minijinja::new();

    let files = TEMPLATE_DIR_FILES.files();
    parse_template(&mut jinja, files);
    let dirs = TEMPLATE_DIR_FILES.dirs();
    dirs.into_iter().for_each(|dir| {
        parse_template_dir(&mut jinja, dir);
    });
    jinja
}

#[cfg(not(feature = "hot-reload"))]
fn parse_template_dir(jinja: &mut Minijinja<'static>, dir: &include_dir::Dir<'static>) {
    let files = dir.files();
    parse_template(jinja, files);
    dir.dirs().into_iter().for_each(|dir| {
        parse_template_dir(jinja, dir);
    });
}

#[cfg(not(feature = "hot-reload"))]
fn parse_template<I: Iterator<Item = &'static include_dir::File<'static>>>(
    jinja: &mut Minijinja<'static>,
    files: I,
) {
    use std::ffi::OsStr;

    files
        .into_iter()
        .map(|f| (f.path(), f.contents_utf8()))
        .filter(|(path, _contents)| {
            path.extension()
                .and_then(OsStr::to_str)
                .unwrap_or("")
                .ends_with("jinja")
        })
        .map(|(path, contents)| {
            (
                path.to_str()
                    .unwrap_or_else(|| panic!("template path to_str() failed")),
                contents.unwrap_or_else(|| panic!("template contents not utf8")),
            )
        })
        .for_each(
            |(path, contents)| match jinja.add_template(path, contents) {
                Ok(c) => c,
                Err(e) => panic!("{path} template couldn't be added: {e}"),
            },
        );
}
