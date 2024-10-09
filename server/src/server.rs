use std::env;
use std::future::Future;
use std::net::Ipv4Addr;

use anyhow::Context;
use anyhow::Result;
use axum_session::DatabasePool;
use axum_session::SessionConfig;
use axum_session::SessionLayer;
use axum_session::SessionStore;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::error;
use tracing::info;
use tracing::trace;

use crate::session::api_database_pool;
use crate::session::api_database_pool::ApiDatabasePool;
use crate::session::get_api_storage;
#[cfg(feature = "log")]
use crate::tracing_layer::TracingLayer;

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

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_server_main<F: Future<Output = ()> + Send + 'static>(
    shutdown: Option<F>,
) -> Result<()> {
    let listener = get_tcp_listener().await?;

    // let app = endpoint::get_route().await?;
    let app = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/2", axum::routing::get(|| async { "Hello, World2!" }));

    let pool = get_api_storage("http://127.0.0.1:8000".to_string()).await?;
    let session_config = SessionConfig::default();
    let session_storage = SessionStore::<ApiDatabasePool>::new(Some(pool), session_config).await?;

    let session_layer = SessionLayer::new(session_storage);

    //todo session layer destroys app! (no endpoint reachable if session layer is added and session server unreachable!)
    //todo this needs to be logged and the timeout needs to be short
    //todo also logging is broken in the session layer
    let app_session = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, Session!" }))
        .layer(session_layer);

    let app = app.nest("/session", app_session);

    #[cfg(feature = "log")]
    let app = app.layer(TracingLayer {});

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
