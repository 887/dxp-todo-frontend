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

const TAILWIND_URL: &str = manganis::mg!("public/tailwind.css");

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
        document::Link { rel: "stylesheet", href: TAILWIND_URL }
        Link { to: Route::Blog { id: count() }, "Go to blog" }
        h1 { class: "text-3xl font-bold underline", "Hello world!" }
        div {
            header {
                class: "text-gray-400 body-font",
                // you can use optional attributes to optionally apply a tailwind class
                class: if true { "bg-gray-900" },
                div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                    a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                        span { class: "ml-3 text-xl", "Hello Dioxus!" }
                    }
                    nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                        a { class: "mr-5 hover:text-white", "First Link" }
                        a { class: "mr-5 hover:text-white", "Second Link" }
                        a { class: "mr-5 hover:text-white", "Third Link" }
                        a { class: "mr-5 hover:text-white", "Fourth Link" }
                    }
                    button { class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                        "Button"
                    }
                }
            }
            h1 { class: "bg-blue", "High-Five counter: {count}" }
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
