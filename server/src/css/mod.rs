#[cfg(feature = "hot-reload")]
mod watcher;
use tracing::{error, info};
#[cfg(feature = "hot-reload")]
pub use watcher::*;

mod initializer;
pub use initializer::*;

use anyhow::Result;

pub static STYLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/css/");
pub static STYLE_SHEET_OUTPUT_DIR: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/src/routes/static/");
pub static STYLES: [&str; 2] = ["style.css", "layout.css"];

pub static CSS_BUILDER: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tools/css-builder/target/release/css-builder"
);

pub async fn run_bundler() -> Result<()> {
    let styles = STYLES;
    for style in styles {
        let result = run_css_builder(style).await;
        match result {
            Ok(output) => {
                info!("stylesheet {} rebuild", style);
                if !output.is_empty() {
                    info!("{output}");
                }
            }
            Err(e) => error!("failed to rebuild stylesheet {}: {e}", style),
        }
    }
    Ok(())
}
