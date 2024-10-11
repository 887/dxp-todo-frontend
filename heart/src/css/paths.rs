pub static STYLES: [&str; 2] = ["style.css", "layout.css"];
pub static STYLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/css/");
pub static STYLE_SHEET_OUTPUT_DIR: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/src/routes/static/");
pub static CSS_BUILDER: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tools/css-builder/target/release/css-builder"
);
