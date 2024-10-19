#![allow(non_snake_case)]

use std::fmt::format;

use backend::ClientSessionExt;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use server_fn::client;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link { to: Route::Blog { id: count() }, "Go to blog" }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down lows!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            button {
                onclick: move |_| async move {
                    match call_backend().await {
                        Ok(data) => {
                            tracing::info!("Backend call successful: {}", data);
                            let data = format!("Server data: {}", data);
                            text.set(data.to_string());
                        }
                        Err(err) => tracing::error!("Backend call failed: {}", err),
                    }
                },
                "Reset text"
            }
            p { "Server data: {text}" }
        }
    }
}

async fn call_backend() -> anyhow::Result<i64> {
    let api = "http://localhost:3000";
    let client = backend::Client::new(&api);
    Ok(client
        .count()
        .table_name("session")
        .send()
        .await
        .map(|res| res.into_inner().count)?)
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    server::post_server_data(data)
        .await
        .map_err(|err| ServerFnError::ServerError(format!("{:?}", err)))
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    server::get_server_data()
        .await
        .map_err(|err| ServerFnError::ServerError(format!("{:?}", err)))
}
