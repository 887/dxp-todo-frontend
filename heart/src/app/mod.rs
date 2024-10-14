use dioxus::hooks::use_signal;
use dioxus::prelude::rsx;
use dioxus::prelude::*;
// use serde::{Deserialize, Serialize};

pub(crate) fn app() -> Element {
    let user_name = use_signal(|| "?".to_string());
    let permissions = use_signal(|| "?".to_string());

    rsx! {
        div {
            button { onclick: move |_| { async move {} }, "Login Test User" }
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
