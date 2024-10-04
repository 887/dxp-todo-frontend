#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use std::sync::Arc;

use hot_lib_reloader::BlockReload;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, RwLock};
use tokio::{sync::mpsc, task::spawn_blocking};
use tracing::error;
use tracing::trace;

use super::hot;

pub async fn run(
    tx_shutdown_server: Arc<RwLock<Option<Sender<()>>>>,
    block_reloads_mutex: Arc<Mutex<()>>,
) {
    //communication channels must outlive the loop
    let (tx_lib_reloaded, mut rx_lib_reloaded) = mpsc::channel(1);

    loop {
        let lib_reloaded_ready = lib_ready_to_reload(
            "server",
            &mut rx_lib_reloaded,
            &tx_shutdown_server,
            &block_reloads_mutex,
            || hot::hot_server::subscribe().wait_for_reload(),
        );

        let observe_lib_hot = observe_lib(
            "tx_lib_reloaded_hot",
            || hot::hot_server::subscribe().wait_for_about_to_reload(),
            &tx_lib_reloaded,
        );

        tokio::select! {
            _ = lib_reloaded_ready => {},
            _ = observe_lib_hot => {},
        };
    }
}

async fn lib_ready_to_reload(
    context_desc: &str,
    rx_lib_reloaded: &mut Receiver<BlockReload>,
    tx_shutdown_server: &Arc<RwLock<Option<Sender<()>>>>,
    block_reloads_mutex: &Arc<Mutex<()>>,
    wait_for_reload: impl Fn() + Send + Sync + 'static,
) {
    let Some(br) = rx_lib_reloaded.recv().await else {
        trace!("reload observer channel for {context_desc} closed");
        return;
    };

    trace!(">>>> {context_desc} reload");

    signal_server_to_shutdown(tx_shutdown_server).await;

    //wait for server to shut down, by waiting on this mutex
    let lock = block_reloads_mutex.lock().await;
    trace!("---{context_desc} reloading---");

    drop(br);

    do_reload(wait_for_reload).await;

    trace!("---{context_desc} reload finished---");
    drop(lock);
}

async fn observe_lib(
    context_desc: &str,
    wait: impl Fn() -> BlockReload + Send + Sync + 'static,
    tx_lib_reloaded_hot: &Sender<BlockReload>,
) {
    if let Some(br) = wait_for_reload(wait).await {
        if let Err(e) = tx_lib_reloaded_hot.send(br).await {
            trace!("error sending {context_desc} signal: {:?}", e);
        }
    }
}

async fn signal_server_to_shutdown(tx_shutdown_server: &Arc<RwLock<Option<Sender<()>>>>) {
    if let Some(server_sender) = tx_shutdown_server.write().await.as_mut() {
        trace!("send shutdown to server!");
        if let Err(err) = (server_sender).send(()).await {
            error!("error sending shutdown signal: {}", err);
        }
    }
}

async fn wait_for_reload(
    f: impl Fn() -> BlockReload + Send + Sync + 'static,
) -> Option<BlockReload> {
    let block_reload_result = spawn_blocking(f).await;
    match block_reload_result {
        Ok(br) => Some(br),
        Err(err) => {
            error!("wait_for_about_to_reload error: {:?}", err);
            None
        }
    }
}

async fn do_reload(wait_for_reload: impl Fn() + Send + Sync + 'static) {
    // Now we wait for the lib to be reloaded...
    let reload_result = spawn_blocking(wait_for_reload).await;
    match reload_result {
        Ok(_) => {
            trace!("reload successful")
        }
        Err(err) => {
            error!("reload error: {:?}", err)
        }
    }
}
