use derivative::Derivative;
use minijinja::Environment as Minijinja;
#[cfg(feature = "hot-reload")]
use std::sync::Arc;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct State {
    #[cfg(not(feature = "hot-reload"))]
    pub templates: &'static Minijinja<'static>,
    #[cfg(feature = "hot-reload")]
    pub templates: &'static arc_swap::ArcSwap<Minijinja<'static>>,
}

impl State {
    #[cfg(not(feature = "hot-reload"))]
    pub fn get_templates(&self) -> &'static Minijinja<'static> {
        self.templates
    }
    #[cfg(feature = "hot-reload")]
    pub fn get_templates(&self) -> arc_swap::Guard<Arc<Minijinja<'static>>> {
        self.templates.load()
    }
}
