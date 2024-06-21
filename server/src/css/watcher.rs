use notify::Event;
use std::ffi::OsStr;

pub async fn handle_event(event: Event, dir: &str) {
    let paths = event
        .paths
        .iter()
        .filter(|p| p.extension().unwrap_or(OsStr::new("")) == "jinja");

    //this remove the cached template, so it will be loaded again when we access it
    for p in paths {
        // let name = p
        //     .to_string_lossy()
        //     .chars()
        //     .skip(dir.chars().count())
        //     .collect::<String>();

        let styles = super::STYLES;
        for style in styles {
            let result = super::run_css_builder(style).await;
            match result {
                Ok(output) => {
                    println!("stylesheet {} rebuild", style);
                    if output.is_empty() {
                        println!("{output}");
                    }
                }
                Err(e) => println!("failed to rebuild stylesheet {}: {e}", style),
            }
        }
    }
}
