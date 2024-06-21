use notify::Event;
use std::{ffi::OsStr, sync::Arc};
use tracing::{error, trace};

use super::{initializer, I18NResourcesType};

pub async fn handle_event(event: Event, dir: &str, container: &I18NResourcesType) {
    let paths = event
        .paths
        .iter()
        .filter(|p| p.extension().unwrap_or(OsStr::new("")) == "ftl");

    let mut any = true;
    //this remove the cached template, so it will be loaded again when we access it
    for p in paths {
        let name = p
            .to_string_lossy()
            .chars()
            .skip(dir.chars().count())
            .collect::<String>();
        trace!("reloading template: {}", &name);

        any = true;
    }

    if any {
        let i18n_maybe = initializer::get_i18n_data();
        match i18n_maybe {
            Ok(i18n) => {
                let arc = Arc::new(i18n);
                container.swap(arc);
            }
            Err(err) => error!("error loading i18n data: {:?}", err),
        }
    }
}
