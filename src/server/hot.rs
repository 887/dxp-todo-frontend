use tokio::sync::mpsc::Receiver;

use tracing::error;

use super::{get_log_subscription, observe};

//info: in order to cause a reload you nee to actually change a function signature/make the compiler do work
//if the file is identical to the compiler, hot-reload will not try to do a reload

#[hot_lib_reloader::hot_module(dylib = "server", file_watch_debounce = 10)]
pub(crate) mod hot_server {
    // pub use lib::*;
    pub type Result<T> = crate::Result<T>;

    hot_functions_from_file!("server/src/hot.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    use std::sync::Arc;
    use tokio::sync::mpsc;
    use tokio::sync::{Mutex, RwLock};
    use tracing::trace;

    dotenvy::dotenv()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "could not load .env"))?;

    #[cfg(feature = "path-info")]
    path_info::print_paths();

    //this channel is to shut down the server
    let tx_shutdown_server = Arc::new(RwLock::new(None));

    //ensures that the server and reloads are blocking
    let block_reloads_mutex = Arc::new(Mutex::new(()));

    tokio::task::spawn({
        let block_reload_mutex = Arc::clone(&block_reloads_mutex);
        let tx_shutdown_server = Arc::clone(&tx_shutdown_server);
        async move {
            #[cfg(feature = "log")]
            let Ok(log_guard) = get_log_subscription() else {
                return;
            };
            let res = observe::run(tx_shutdown_server, block_reload_mutex).await;
            #[cfg(feature = "log")]
            drop(log_guard);
            res
        }
    });

    //main loop
    loop {
        //only run when we can access the mutex
        let lock = block_reloads_mutex.lock().await;

        let (tx, rx_shutdown_server) = mpsc::channel(1);
        {
            let mut lock = tx_shutdown_server.write().await;
            *lock = Some(tx);
        }

        #[cfg(feature = "log")]
        let log_guard = get_log_subscription()?;

        trace!("---main loop---");

        run(rx_shutdown_server).await;

        trace!("---main loop finished---");

        #[cfg(feature = "log")]
        drop(log_guard);

        //only allow more reloads once finished
        drop(lock);
    }
}

//everything that can fail needs to be in this task
//once this task finishes the hot-reload-lib checks if there is a new library to reload
pub(crate) async fn run(rx_shutdown_server: Receiver<()>) {
    if let Err(err) = run_inner(rx_shutdown_server).await {
        error!("running main_task failed: {:?}", err);
        error!("waiting 3s..");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

async fn run_inner(rx_shutdown_server: Receiver<()>) -> crate::Result<()> {
    hot_server::load_env()?;

    run_server(rx_shutdown_server).await
}

async fn run_server(rx_shutdown_server: Receiver<()>) -> crate::Result<()> {
    // use std::{thread};
    // thread::spawn(|| {
    // }).join() {

    // https://stackoverflow.com/a/62536772
    // the tokio threadpool is used here

    Ok(tokio::task::spawn_blocking(|| {
        hot_server::run_server(rx_shutdown_server)
            .map_err(|e| format!("run_server aborted with error: {:?}", e))
    })
    .await??)
}
