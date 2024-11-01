#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{MediaStream, MediaStreamConstraints};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

// const TAILWIND_URL: &str = manganis::mg!("public/tailwind.css");
const TAILWIND_URL: &str = asset!("public/tailwind.css");

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

//this is video download, not screen share
//https://github.com/DioxusLabs/dioxus/blob/main/examples/video_stream.rs

//https://github.com/DioxusLabs/dioxus/blob/main/examples/todomvc.rs

//todo
//try to get screen share going

#[component]
fn ScreenShare() -> Element {
    let video_player_id = "video_player";
    let window = match web_sys::window() {
        Some(win) => win,
        None => {
            tracing::error!("Unable to access window");
            return rsx! {
                div { "Error: Unable to access window" }
            };
        }
    };

    let document = match window.document() {
        Some(doc) => doc,
        None => {
            tracing::error!("Unable to access document");
            return rsx! {
                div { "Error: Unable to access document" }
            };
        }
    };

    let start_screen_share = move |_| {
        let video_ref = match document.get_element_by_id(video_player_id) {
            Some(element) => element,
            None => {
                tracing::error!("Unable to find video element");
                return;
            }
        };
        let video_element = match video_ref.dyn_into::<web_sys::HtmlVideoElement>() {
            Ok(element) => element,
            Err(_) => {
                tracing::error!("Unable to cast video element");
                return;
            }
        };
        let media_devices = match window.navigator().media_devices() {
            Ok(devices) => devices,
            Err(_) => {
                tracing::error!("Unable to access media devices");
                return;
            }
        };

        let constraints = MediaStreamConstraints::new();
        constraints.set_video(&JsValue::TRUE);

        let promise = match media_devices.get_display_media() {
            Ok(promise) => promise,
            Err(err) => {
                tracing::error!("Error getting display media: {:?}", err);
                return;
            }
        };

        let future = wasm_bindgen_futures::JsFuture::from(promise);
        wasm_bindgen_futures::spawn_local(async move {
            match future.await {
                Ok(stream) => {
                    let media_stream = MediaStream::from(stream);
                    video_element.set_src_object(Some(&media_stream));
                }
                Err(err) => {
                    tracing::error!("Error starting screen share: {:?}", err);
                }
            }
        });
    };

    let var_name = rsx! {
        div {
            button {
                onclick: start_screen_share,
                class: "bg-blue-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                "Start Screen Share"
            }
            video {
                id: video_player_id,
                autoplay: true,
                controls: true,
                style: "width: 100%; height: auto;"
            }
        }
    };

    var_name
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    let mut show_element = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_URL }
        Link { to: Route::Blog { id: count() }, "Go to blog" }
        h1 { class: "text-3xl font-bold underline", "Hello world!" }
        div {
            //example code https://github.com/DioxusLabs/dioxus/blob/main/examples/tailwind/src/main.rs
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
                class: "bg-blue-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        if let Err(err) = post_server_data(data).await {
                            tracing::error!("Error posting server data: {:?}", err);
                        }
                    }
                },
                "Get Server Data"
            }

            button {
                class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| {
                    let value = show_element();
                    show_element.set(!value)
                },
                "Toggle Element"
            }

            if (*show_element)() {
                div { "This is the new element!" }
                ScreenShare {}
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
