use poem::Route;

#[cfg(not(feature = "hot-reload"))]
use poem::endpoint::EmbeddedFilesEndpoint;
#[cfg(feature = "hot-reload")]
use poem::endpoint::StaticFilesEndpoint;
#[cfg(not(feature = "hot-reload"))]
use rust_embed::RustEmbed;

pub fn get_route() -> Route {
    let route = Route::new();
    //the static-files-endpoint goes through the filesystem on runtime and looks for static files
    #[cfg(feature = "hot-reload")]
    let route = {
        //println!("Serving static files at /static");
        route.nest(
            "/",
            StaticFilesEndpoint::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/routes/static")),
        )
    };

    //the embedded-files-endpoint embeds them when we're building
    #[cfg(not(feature = "hot-reload"))]
    let route = {
        //println!("Serving embedded files at /static");
        route.nest("/", EmbeddedFilesEndpoint::<StaticFiles>::new())
    };

    route
}
