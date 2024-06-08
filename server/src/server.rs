use std::env;
use std::future::Future;

use anyhow::Context;
use anyhow::Result;

use poem::IntoEndpoint;
use poem::{listener::TcpListener, Server};
use tracing::error;
use tracing::info;
use tracing::trace;

use crate::endpoints;

pub fn get_tcp_listener() -> Result<TcpListener<String>> {
    let host = env::var("HOST").context("HOST is not set")?;
    let port = env::var("PORT").context("PORT is not set")?;

    let server_url = format!("http://{host}:{port}");

    info!("Starting server at {server_url}");

    Ok(TcpListener::bind(format!("{host}:{port}")))
}

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_server_main<F: Future<Output = ()>>(shutdown: Option<F>) -> Result<()> {
    let tcp_listener = get_tcp_listener()?;
    let endpoints = endpoints::get_route().await?;

    let server = Server::new(tcp_listener);

    info!("running sever");

    let run_result = match shutdown {
        Some(shutdown) => {
            server
                .run_with_graceful_shutdown(endpoints, shutdown, None)
                .await
        }
        None => server.run(endpoints).await,
    };

    let result = match run_result {
        Ok(_) => {
            trace!("server shut down success");
            Ok(())
        }
        Err(err) => {
            error!("server shut down with error: {:?}", err);
            Err(anyhow::anyhow!("server error: {}", err))
        }
    };

    result
}
