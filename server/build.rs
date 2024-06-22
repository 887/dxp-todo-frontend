#[cfg(not(feature = "github"))]
use std::process::Command;
use std::{io::Read, path::Path};

#[path = "src/css/paths.rs"]
mod paths;
use paths::*;

use is_executable::IsExecutable;

fn main() {
    #[cfg(not(feature = "github"))]
    let current_manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap().to_string();
    #[cfg(not(feature = "github"))]
    let css_builder = current_manifest + "/tools/css-builder";

    //this line means we only trigger a rebuild if the mtime of a file changes..
    //we depend on multiple and have hot reload, should be fine without this (for now)
    // #[cfg(not(feature = "github"))]
    // println!("cargo:rerun-if-changed={}", file?);

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

        let styles = STYLES;
        for style in styles {
            let result = run_css_builder(style);
            match result {
                Ok(output) => {
                    // info!("stylesheet {} rebuild", style);
                    if !output.is_empty() {
                        println!("failed to rebuild stylesheet {output}");
                    }
                }
                Err(e) => println!("failed to rebuild stylesheet {}: {e}", style),
            }
        }
    }
}

fn run_bundler_command(sheet_file_name: &str) -> Result<std::process::Output, std::io::Error> {
    let p = Path::new(CSS_BUILDER);
    if !(p.is_executable()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("File not executable {}", CSS_BUILDER),
        ));
    }

    //https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output
    let style_sheet = STYLE_DIR.to_string() + sheet_file_name;
    let sheet_path = Path::new(&style_sheet);
    let style_sheet_output = STYLE_SHEET_OUTPUT_DIR.to_string() + sheet_file_name;
    let sheet_output_path = Path::new(&style_sheet_output);
    Command::new(p)
        .arg("--style-sheet")
        .arg(sheet_path)
        .arg("--output")
        .arg(sheet_output_path)
        .output()
}

fn run_css_builder(stylesheet_name: &str) -> Result<String, String> {
    let result = match run_bundler_command(stylesheet_name) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    if !result.status.success() {
        match String::from_utf8(result.stderr) {
            Ok(stderr_output) => return Err(stderr_output),
            Err(err) => return Err(err.to_string()),
        };
    }

    let css_maybe = String::from_utf8(result.stdout);
    match css_maybe {
        Ok(css) => Ok(css),
        Err(err) => Err(err.to_string()),
    }
}
