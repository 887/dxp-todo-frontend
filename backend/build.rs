#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use regex::Regex;
use std::io::Write;

use progenitor::GenerationSettings;

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
    let mut content = std::fs::read_to_string(swagger_file).unwrap();

    //progenitor only supports openapi 3.0
    //https://github.com/oxidecomputer/progenitor/issues/762

    //we can get around this by patching the file back to 3.0

    #[cfg(feature = "json")]
    if content.contains(r#""openapi": "3.1.0","#) {
        let re = regex::Regex::new(r#""type":\s*\[\s*"string",\s*"null"\s*\]"#).unwrap();
        let patched_content = re.replace_all(&content, r#""type": "string", "nullable": true"#);
        let patched_content =
            patched_content.replace(r#""openapi": "3.1.0","#, r#""openapi": "3.0.3","#);
        let patched_file = swagger_file.to_string() + ".patched";
        std::fs::write(patched_file, patched_content.as_bytes()).unwrap();

        content = patched_content.to_string();
    }

    #[cfg(feature = "json")]
    let spec = serde_json::from_str(&content).unwrap();

    //TODO patch the yaml file like the json file

    #[cfg(feature = "yaml")]
    let mut spec = serde_yml::from_str(&content).unwrap();

    let mut settings = GenerationSettings::default();
    settings.with_interface(progenitor::InterfaceStyle::Builder);
    settings.with_tag(progenitor::TagStyle::Separate);
    let mut generator = progenitor::Generator::new(&settings);
    generator.uses_futures();

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
