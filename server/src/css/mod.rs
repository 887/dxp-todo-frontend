#[cfg(feature = "hot-reload")]
mod watcher;
#[cfg(feature = "hot-reload")]
pub use watcher::*;

mod initializer;
pub use initializer::*;

pub static STYLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/server/src/css/");
pub static STYLE_SHEET_OUTPUT_DIR: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/server/routes/static/");
pub static STYLES: [&str; 2] = ["style.css", "layout.css"];

pub static CSS_BUILDER: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tools/css-builder/target/release/css-builder"
);
