use crate::app::app;

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

    dioxus::launch(app);

    // Hydrate the application on the client
    dioxus_web::launch::launch_cfg(app, dioxus_web::Config::new().hydrate(true));

    Ok(())
}

mod app {}
