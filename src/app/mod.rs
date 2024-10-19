#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

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
                    let data = call_backend_with_server().await;
                    match data {
                        Ok(data) => {
                            tracing::info!("Client received: {}", data);
                            text.set(data.clone());
                        }
                        Err(err) => {
                            tracing::error!("Client error: {:?}", err);
                            text.set("Error".to_string());
                        }
                    }
                },
                "Reset text"
            }
            p { "Server data: {text}" }
        }
    }
}

#[server(CallBackend)]
async fn call_backend_with_server() -> Result<String, ServerFnError> {
    server::call_backend_with_server()
        .await
        .map(|data| data.to_string())
        .map_err(|err| ServerFnError::ServerError(format!("{:?}", err)))
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
