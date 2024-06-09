use notify::Event;
use std::{ffi::OsStr, sync::Arc};
use tracing::trace;

use super::{initializer, I18NResourcesType};

pub fn handle_event(event: Event, dir: &str, container: &I18NResourcesType) {
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
        if let Ok(i18n) = i18n_maybe {
            let arc = Arc::new(i18n);
            container.swap(arc);
        } else if let Err(err) = i18n_maybe {
            println!("watch error: {err:?}");
        }
    }
}
