use arc_swap::ArcSwap;
use minijinja::Environment as Minijinja;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::OnceLock;
use std::{ffi::OsStr, path::Path};
use tokio::sync::mpsc::{self, Receiver};

use crate::initializers::templates;

pub fn watch_directory(
    templates_dir: &'static str,
    templates: &'static OnceLock<ArcSwap<Minijinja<'static>>>,
) {
    //https://old.reddit.com/r/rust/comments/q6nyc6/async_file_watcher_like_notifyrs/
    tokio::spawn(async move {
        loop {
            // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            if let Err(e) = async_watch(std::path::Path::new(templates_dir), templates).await {
                println!("error: {:?}", e)
            }
        }
    });
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = mpsc::channel(1);

    let handle = tokio::runtime::Handle::current();
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            handle.block_on(async {
                if tx.send(res).await.ok().is_none() {
                    println!("error on template future execution");
                }
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(
    path: P,
    templates_container: &'static OnceLock<ArcSwap<Minijinja<'static>>>,
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
                    println!("reloading templates: {event:?}");

                    //this reloads all files from disk
                    let templates = templates::get_templates();

                    templates_container
                        .get()
                        .map(|container| container.swap(std::sync::Arc::new(templates)));

                    println!("templates reloaded");
                }
            }
            Err(e) => println!("watch error: {e:?}"),
        }
    }

    Ok(())
}
