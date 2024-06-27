use notify::Event;
use std::ffi::OsStr;
use tracing::{error, info, trace};

pub async fn handle_event(event: Event, dir: &str) {
    let paths = event
        .paths
        .iter()
        .filter(|p| p.extension().unwrap_or(OsStr::new("")) == "css");

    let styles = super::STYLES;

    //this remove the cached template, so it will be loaded again when we access it
    for p in paths {
        let name = p
            .to_string_lossy()
            .chars()
            .skip(dir.chars().count())
            .collect::<String>();

        trace!("rebuilding stylesheet: {}", &name);

        let filename: &str = &p
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or(String::new());
        if !styles.contains(&filename) {
            continue;
        }

        let result = super::run_css_builder(filename).await;
        match result {
            Ok(output) => {
                info!("stylesheet {} rebuild", filename);
                if !output.is_empty() {
                    info!("{output}");
                }
            }
            Err(e) => error!("failed to rebuild stylesheet {}: {e}", filename),
        }
    }
}
