use std::process::Command;

fn main() {
    let current_manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap().to_string();
    let css_builder = current_manifest.clone() + "/tools/css-builder";
    let css_builder_manifest = css_builder.clone() + "/Cargo.toml";

    println!("cargo:rerun-if-changed={}", css_builder_manifest);

    Command::new("cargo")
        .arg("build")
        .current_dir(css_builder)
        .spawn()
        .expect("cargo build command failed to start");

    // let tools = current_manifest.clone() + "/tools";
    // let tools = std::path::Path::new(&tools);
    // std::fs::create_dir(tools);
    // escargot::CargoBuild::new()
    //     .bin("bin")
    //     .current_release()
    //     .current_target()
    //     .manifest_path(css_builder_manifest)
    //     .target_dir(tools)
    //     .exec()
    //     .unwrap();
}
