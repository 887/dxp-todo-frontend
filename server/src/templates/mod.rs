#[cfg(feature = "hot-reload")]
pub mod watcher;

pub mod initializer;

#[cfg(feature = "hot-reload")]
use arc_swap::ArcSwap;
use minijinja::Environment as Minijinja;
use std::sync::OnceLock;

#[cfg(feature = "hot-reload")]
static TEMPLATES: OnceLock<ArcSwap<Minijinja<'static>>> = OnceLock::new();
#[cfg(feature = "hot-reload")]
pub fn get_templates() -> &'static arc_swap::ArcSwapAny<std::sync::Arc<Minijinja<'static>>> {
    TEMPLATES.get_or_init(|| {
        let templates = initializer::get_templates();
        //https://docs.rs/arc-swap/latest/arc_swap/index.html
        ArcSwap::from_pointee(templates)
    })
}

#[cfg(not(feature = "hot-reload"))]
static TEMPLATES: OnceLock<Minijinja<'static>> = OnceLock::new();
#[cfg(not(feature = "hot-reload"))]
pub fn get_templates() -> &'static Minijinja<'static> {
    TEMPLATES.get_or_init(|| templates::get_templates_embedded())
}
