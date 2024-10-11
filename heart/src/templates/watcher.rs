use notify::Event;
use std::ffi::OsStr;
use tracing::trace;

use super::TemplatesType;

pub async fn handle_event(event: Event, dir: &str, container: &'static TemplatesType) {
    let paths = event
        .paths
        .iter()
        .filter(|p| p.extension().unwrap_or(OsStr::new("")) == "jinja");

    //this reloads all the templates
    // if jinja_paths.count() > 0 {
    //     trace!("reloading templates: {event:?}");

    //     //this reloads all files from disk
    //     let templates = initializer::get_templates();
    //     templates_container.swap(std::sync::Arc::new(templates));

    //     trace!("templates reloaded");
    // }

    //this remove the cached template, so it will be loaded again when we access it
    for p in paths {
        let name = p
            .to_string_lossy()
            .chars()
            .skip(dir.chars().count())
            .collect::<String>();

        let tps_arc = container.load_full();
        let mut tps = (*tps_arc).clone(); //get a mutable copy of our templates collection

        // let remove_template = |tps: &mut minijinja::Environment<'static>| {
        trace!("reloading template: {}", &name);
        //this template does not have to be loaded, remove templates does not check if something exists
        tps.remove_template(&name);

        // };

        // match tps.get_template(&name) {
        //     Ok(_) => remove_template(&mut tps),
        //     Err(e) => match e.kind() {
        //         minijinja::ErrorKind::TemplateNotFound => {}
        //         _ => remove_template(&mut tps),
        //     },
        // }

        container.swap(std::sync::Arc::new(tps));
    }
}
