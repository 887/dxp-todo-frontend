use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::{Deserialize, Serialize};

//https://github.com/dxps/fullstack-rust-axum-dioxus-rwa/tree/main/frontend
//https://docs.rs/dioxus-fullstack/latest/dioxus_fullstack/
//https://crates.io/crates/dioxus-hot-reload

pub fn main() -> std::io::Result<()> {
    println!(
        "web started \n\
        \n\
        \n\
        "
    );

    // Hydrate the application on the client
    dioxus_web::launch::launch_cfg(app, dioxus_web::Config::new().hydrate(true));

    Ok(())
}

fn app() -> Element {
    let user_name = use_signal(|| "?".to_string());
    let permissions = use_signal(|| "?".to_string());

    rsx! {
        div {
            button { onclick: move |_| {
                    async move {
                        // login().await.unwrap();
                    }
                },
                "Login Test User"
            }
        }
        div {
            button {
                onclick: move |_| async move {
                    // if let Ok(data) = get_user_name().await {
                    //     user_name.set(data);
                    // }
                },
                "Get User Name"
            }
            "User name: {user_name}"
        }
        div {
            button {
                onclick: move |_| async move {
                    // if let Ok(data) = get_permissions().await {
                    //     permissions.set(data);
                    // }
                },
                "Get Permissions"
            }
            "Permissions: {permissions}"
        }
    }
}
