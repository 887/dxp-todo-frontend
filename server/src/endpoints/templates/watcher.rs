use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{ffi::OsStr, path::Path};
use tokio::sync::mpsc::{self, Receiver};
use tracing::{error, trace};

use super::TemplatesType;

//TODO: create generic watcher with that only needs the event filter

pub fn watch_directory(dir: &'static str, templates: &'static TemplatesType) {
    //https://old.reddit.com/r/rust/comments/q6nyc6/async_file_watcher_like_notifyrs/
    tokio::task::spawn(async move {
        #[cfg(feature = "log")]
        let Ok(log_subscription) = dxp_logging::get_subscription() else {
            return;
        };

        async {
            loop {
                // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                if let Err(e) = async_watch(dir, std::path::Path::new(dir), templates).await {
                    error!("error watching template reload: {:?}", e)
                }
            }
        }
        .await;

        #[cfg(feature = "log")]
        drop(log_subscription);
    });
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = mpsc::channel(1);

    let handle = tokio::runtime::Handle::current();
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            handle.block_on(async {
                if tx.send(res).await.ok().is_none() {
                    error!("template reload notify channel closed - can't send");
                }
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(
    dir: &'static str,
    path: P,
    container: &'static TemplatesType,
) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => {
                if event.kind
                    == notify::EventKind::Modify(notify::event::ModifyKind::Data(
                        notify::event::DataChange::Any,
                    ))
                {
                    continue;
                }

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
            Err(e) => error!("watch error: {e:?}"),
        }
    }

    Ok(())
}
