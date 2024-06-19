use std::io::Read;
#[cfg(not(feature = "github"))]
use std::process::Command;

fn main() {
    #[cfg(not(feature = "github"))]
    let current_manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap().to_string();
    #[cfg(not(feature = "github"))]
    let css_builder = current_manifest.clone() + "/tools/css-builder";
    #[cfg(not(feature = "github"))]
    let css_builder_manifest = css_builder.clone() + "/Cargo.toml";

    //this line means we only trigger a rebuild if the manifest of the submodule changes
    #[cfg(not(feature = "github"))]
    println!("cargo:rerun-if-changed={}", css_builder_manifest);

    //with github do not run this pre-build command, otherwise the github action hangs!
    #[cfg(not(feature = "github"))]
    {
        let output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(css_builder)
            .spawn()
            .expect("cargo build command failed to start");

        if let Some(mut std_err) = output.stderr {
            let mut std_err_out = String::new();
            if let Ok(size) = std_err.read_to_string(&mut std_err_out) {
                if size > 0 {
                    println!("cargo::warning={}", std_err_out);
                    panic!("{}", std_err_out);
                }
            }
        }
    }
}
