use dioxus::hooks::use_signal;
use dioxus::prelude::rsx;
use dioxus::prelude::*;
// use serde::{Deserialize, Serialize};

pub(crate) fn app() -> Element {
    rsx! {
        Elements {}
    }
}

#[component]
fn Elements() -> Element {
    let user_name = use_signal(|| "?".to_string());
    let permissions = use_signal(|| "?".to_string());

    rsx! {
        div {
            div {
                button { onclick: move |_| { async move {} }, "Login Tests User" }
                button { onclick: move |_| { async move {} }, "Login Tests User" }
            }
            div {
                button { onclick: move |_| async move {}, "Get User Name" }
                "User name: {user_name}"
            }
            div {
                button { onclick: move |_| async move {}, "Get Permissions" }
                "Permissions: {permissions}"
            }
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let text = use_signal(|| String::from("..."));

    rsx! {
        Link { to: Route::Blog { id: count() }, "Go to blog" }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button { onclick: move |_| async move {}, "Get Server Data" }
            p { "Server data: {text}" }
        }
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}
