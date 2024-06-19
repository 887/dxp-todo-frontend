#[cfg(not(feature = "github"))]
use std::process::Command;

fn main() {
    #[cfg(not(feature = "github"))]
    let current_manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap().to_string();
    #[cfg(not(feature = "github"))]
    let css_builder = current_manifest.clone() + "/tools/css-builder";
    #[cfg(not(feature = "github"))]
    let css_builder_manifest = css_builder.clone() + "/Cargo.toml";

    #[cfg(not(feature = "github"))]
    println!("cargo:rerun-if-changed={}", css_builder_manifest);

    //with github do not run this pre-build command, otherwise the github action hangs!
    #[cfg(not(feature = "github"))]
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(css_builder)
        .spawn()
        .expect("cargo build command failed to start");

    // let tools = current_manifest.clone() + "/tools/out";
    // let tools = std::path::Path::new(&tools);
    // std::fs::create_dir(tools);
    // let run = escargot::CargoBuild::new()
    //     .current_target()
    //     .manifest_path(css_builder_manifest)
    //     .target_dir(tools)
    //     .exec()
    //     .unwrap();

    // let result = current_manifest.clone() + "/tools/result";
    // std::fs::write(result.clone(), format!("{}", ""));
    // let mut f = std::fs::File::options().append(true).open(result).unwrap();
    // for m in run {
    //     match m {
    //         Ok(m) => writeln!(&mut f, "{}", m.).unwrap(),
    //         Err(ce) => writeln!(&mut f, "").unwrap(),
    //     }
    // }
}
