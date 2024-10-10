use std::env;
use std::future::Future;
use std::net::Ipv4Addr;

use anyhow::Context;
use anyhow::Result;
use axum_session::SessionConfig;
use axum_session::SessionLayer;
use axum_session::SessionStore;
use tokio::runtime::Builder;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::error;
use tracing::info;
use tracing::trace;

use crate::session::api_database_pool::ApiDatabasePool;
use crate::session::get_api_storage;

// use crate::endpoint;

pub async fn get_tcp_listener() -> Result<TcpListener> {
    let host = env::var("HOST").context("HOST is not set")?;
    let port = env::var("PORT").context("PORT is not set")?;

    let server_url = format!("http://{host}:{port}");

    info!("Starting server at {server_url}");

    let port: u16 = port.parse().context("PORT is not a valid number")?;
    let address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port);

    Ok(TcpListener::bind(&address).await?)
}

#[cfg(feature = "log")]
pub fn run_server_main<F: Future<Output = ()> + Send + 'static>(
    shutdown: Option<F>,
    log_dispatcher: &dxp_logging::LogDispatcher,
) -> Result<()> {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::RwLock;

    let hash_map = Arc::new(RwLock::new(HashMap::new()));
    let log_dispatcher_clone = log_dispatcher.clone();
    let runtime = Builder::new_multi_thread()
        .on_thread_start({
            let hash_map = Arc::clone(&hash_map);
            move || {
                // Initialize thread-local resource for each thread
                let log_guard = dxp_logging::set_thread_default_dispatcher(&log_dispatcher_clone);
                let thread_id = std::thread::current().id();
                if let Ok(mut hash_map) = hash_map.write() {
                    hash_map.insert(thread_id, log_guard);
                } else {
                    error!("Failed to acquire write lock for hash_map");
                }
            }
        })
        .on_thread_stop({
            let hash_map = Arc::clone(&hash_map);
            move || {
                let thread_id = std::thread::current().id();
                if let Ok(mut hash_map) = hash_map.write() {
                    hash_map.remove(&thread_id);
                } else {
                    error!("Failed to acquire write lock for hash_map");
                }
            }
        })
        .enable_all()
        .build()?;

    runtime.block_on(async { run_server_main_inner(shutdown).await })
}

//https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[cfg(not(feature = "log"))]
#[tokio::main]
pub async fn run_server_main<F: Future<Output = ()> + Send + 'static>(
    shutdown: Option<F>,
) -> Result<()> {
    run_server_main_inner(shutdown).await
}

pub async fn run_server_main_inner<F: Future<Output = ()> + Send + 'static>(
    shutdown: Option<F>,
) -> Result<()> {
    let listener = get_tcp_listener().await?;

    // let app = endpoint::get_route().await?;

    let app = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                trace!("hello world");
                "Hello, World!"
            }),
        )
        .route(
            "/2",
            axum::routing::get(|| async {
                trace!("hello world 2");
                "Hello, World2!"
            }),
        );

    let pool = get_api_storage("http://127.0.0.1:8000".to_string()).await?;
    let session_config = SessionConfig::default();
    let session_storage = SessionStore::<ApiDatabasePool>::new(Some(pool), session_config).await?;

    let session_layer = SessionLayer::new(session_storage);

    //session error returns internal server error, we should probs log this
    let app_session = axum::Router::new().route(
        "/",
        axum::routing::get(|| async {
            info!("session route");
            "Hello, Session!"
        }),
    );
    let app_session = app_session.layer(session_layer);

    let app = app.nest("/session", app_session);

    #[cfg(feature = "log")]
    let app = app.layer(TraceLayer::new_for_http());

    // let app = app.layer(axum::middleware::from_fn(log_middleware));

    info!("running sever");

    let server = axum::serve(listener, app);
    let run_result = match shutdown {
        Some(shutdown) => {
            let graceful = server.with_graceful_shutdown(shutdown);
            graceful.await
        }
        None => server.await,
    };

    match run_result {
        Ok(_) => {
            trace!("server shut down success");
            Ok(())
        }
        Err(err) => {
            error!("server shut down with error: {:?}", err);
            Err(anyhow::anyhow!("server error: {}", err))
        }
    }
}

// async fn log_middleware(
//     req: axum::http::Request<axum::body::Body>,
//     next: axum::middleware::Next,
// ) -> Response {
//     let log_guard = dxp_logging::subscribe_thread_with_default();
//     // Create a new span for the request
//     let span =
//         tracing::span!(Level::INFO, "request", method = %req.method(), path = %req.uri().path());
//     let enter = span.enter(); // Enter the span

//     // Log the request
//     info!("Received request");

//     // Call the next middleware/handler
//     let response = next.run(req).await;

//     // Log the response
//     info!("Response: {:?}", response);

//     drop(enter);
//     drop(log_guard);

//     response
// }
