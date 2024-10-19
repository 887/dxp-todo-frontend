#[cfg(feature = "log")]
use super::get_log_subscription;

//info: in order to cause a reload you nee to actually change a function signature/make the compiler do work
//if the file is identical to the compiler, hot-reload will not try to do a reload

#[hot_lib_reloader::hot_module(dylib = "heart", file_watch_debounce = 10)]
pub(crate) mod hot_heart {
    // pub use lib::*;
    pub type Result<T> = crate::Result<T>;

    hot_functions_from_file!("heart/src/hot.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

pub use hot_heart::*;

#[cfg(feature = "log")]
pub fn log_reload() {
    use std::thread;
    use tracing::info;
    thread::spawn(|| {
        #[cfg(feature = "log")]
        let Ok(_log_guard) = get_log_subscription() else {
            return;
        };
        tracing::info!("Waiting for reloads...");
        loop {
            hot_heart::subscribe().wait_for_reload();
            info!("Reloading heart...");
        }
        #[cfg(feature = "log")]
        drop(_log_guard);
    });
}
