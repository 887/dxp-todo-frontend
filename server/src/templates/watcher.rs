use arc_swap::ArcSwap;
use minijinja::Environment as Minijinja;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{ffi::OsStr, path::Path};
use tokio::sync::mpsc::{self, Receiver};
use tracing::{error, trace};

use crate::templates::initializer;

pub fn watch_directory(
    templates_dir: &'static str,
    templates: &'static ArcSwap<Minijinja<'static>>,
) {
    //https://old.reddit.com/r/rust/comments/q6nyc6/async_file_watcher_like_notifyrs/
    tokio::task::spawn(async move {
        #[cfg(feature = "log")]
        let Ok(log_subscription) = dxp_logging::get_subscription() else {
            return;
        };

        async {
            loop {
                // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                if let Err(e) = async_watch(std::path::Path::new(templates_dir), templates).await {
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
    path: P,
    templates_container: &'static ArcSwap<Minijinja<'static>>,
) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => {
                if event
                    .paths
                    .iter()
                    .any(|p| p.extension().unwrap_or(OsStr::new("")) == "jinja")
                {
                    trace!("reloading templates: {event:?}");

                    //this reloads all files from disk
                    let templates = initializer::get_templates();

                    templates_container.swap(std::sync::Arc::new(templates));

                    trace!("templates reloaded");
                }
            }
            Err(e) => error!("watch error: {e:?}"),
        }
    }

    Ok(())
}
