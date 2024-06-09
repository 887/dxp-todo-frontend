use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{ffi::OsStr, path::Path, sync::Arc};
use tokio::sync::mpsc::{self, Receiver};
use tracing::{error, trace};

use super::{initializer, I18NResourcesType};

pub fn watch_directory(dir: &'static str, container: &'static I18NResourcesType) {
    //https://old.reddit.com/r/rust/comments/q6nyc6/async_file_watcher_like_notifyrs/
    tokio::task::spawn(async move {
        #[cfg(feature = "log")]
        let Ok(log_subscription) = dxp_logging::get_subscription() else {
            return;
        };

        async {
            loop {
                // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                if let Err(e) = async_watch(dir, std::path::Path::new(dir), container).await {
                    error!("error watching i18n reload: {:?}", e)
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
    container: &'static I18NResourcesType,
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
            Err(e) => error!("watch error: {e:?}"),
        }
    }

    Ok(())
}
