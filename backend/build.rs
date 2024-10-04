#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use std::io::Write;

//https://github.com/oxidecomputer/progenitor?tab=readme-ov-file#buildrs
fn main() {
    #[cfg(feature = "json")]
    let swagger_file = "swagger.json";
    #[cfg(feature = "yaml")]
    let swagger_file = "swagger.yaml";
    if let Ok(rsp) = reqwest::blocking::get("http://127.0.0.1:8000/api/".to_string() + swagger_file)
    {
        let mut file = std::fs::File::create(swagger_file).unwrap();
        file.write_all(&rsp.bytes().unwrap()).unwrap();
    }

    println!("cargo:rerun-if-changed={}", swagger_file);
    let file = std::fs::File::open(swagger_file).unwrap();
    #[cfg(feature = "json")]
    let spec = serde_json::from_reader(file).unwrap();
    #[cfg(feature = "yaml")]
    let spec = serde_yml::from_reader(file).unwrap();
    let mut generator = progenitor::Generator::default();

    let tokens = generator.generate_tokens(&spec).unwrap();
    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    //https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    let mut out_file =
        std::path::Path::new(&(std::env::var("CARGO_MANIFEST_DIR").unwrap().to_string() + "/src"))
            .to_path_buf();

    let allow = "#![allow(clippy::unwrap_used)]\n\
        #![allow(unused_variables)]\n\
        #![allow(dead_code)]\n"
        .to_string();

    out_file.push("client.rs");

    std::fs::write(out_file, allow + &content).unwrap();
}
