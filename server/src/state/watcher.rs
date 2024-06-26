use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{future::Future, path::Path};
use tokio::sync::mpsc::{self, Receiver};
use tracing::error;

//TODO: solve duplicate code with this:
// https://stackoverflow.com/questions/60345904/defining-a-macro-that-passes-params-to-a-function

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

// Helper trait for calling a function with a list of arguments.
trait WatcherWithArgs<Args> {
    // Return type.
    type EventReturnType;

    /// Call function with an argument list.
    fn watch_directory<F: Send + Sync + Fn(Event, &'static str, Args) -> Self::EventReturnType>(
        &self,
        dir: &'static str,
        process_event: &'static F,
        args: Args,
    );
}

pub fn watch_directory_container<
    C: Send + Sync,
    RetFut: Future<Output = ()> + Send + Sync,
    F: Send + Sync + Fn(Event, &'static str, &'static C) -> RetFut,
>(
    dir: &'static str,
    container: &'static C,
    process_event: &'static F,
) where
    F:,
{
    //https://old.reddit.com/r/rust/comments/q6nyc6/async_file_watcher_like_notifyrs/
    tokio::task::spawn(async move {
        #[cfg(feature = "log")]
        let Ok(log_subscription) = dxp_logging::get_subscription() else {
            return;
        };

        async {
            loop {
                // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                if let Err(e) =
                    async_watch_container(dir, std::path::Path::new(dir), container, process_event)
                        .await
                {
                    error!("error watching i18n reload: {:?}", e)
                }
            }
        }
        .await;

        #[cfg(feature = "log")]
        drop(log_subscription);
    });
}

pub async fn async_watch_container<
    C: Send + Sync,
    RetFut: Future<Output = ()> + Send + Sync,
    F: Send + Sync + Fn(Event, &'static str, &'static C) -> RetFut,
    P: AsRef<Path>,
>(
    dir: &'static str,
    path: P,
    container: &'static C,
    process_event: F,
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

                process_event(event, dir, container).await;
            }
            Err(e) => error!("watch error: {e:?}"),
        }
    }

    Ok(())
}

pub fn watch_directory<
    RetFut: Future<Output = ()> + Send + Sync,
    F: Send + Sync + Fn(Event, &'static str) -> RetFut,
>(
    dir: &'static str,
    process_event: &'static F,
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
                if let Err(e) = async_watch(dir, std::path::Path::new(dir), process_event).await {
                    error!("error watching i18n reload: {:?}", e)
                }
            }
        }
        .await;

        #[cfg(feature = "log")]
        drop(log_subscription);
    });
}

pub async fn async_watch<
    RetFut: Future<Output = ()> + Send + Sync,
    F: Send + Sync + Fn(Event, &'static str) -> RetFut,
    P: AsRef<Path>,
>(
    dir: &'static str,
    path: P,
    process_event: F,
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

                process_event(event, dir).await;
            }
            Err(e) => error!("watch error: {e:?}"),
        }
    }

    Ok(())
}
