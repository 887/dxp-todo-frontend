use std::io::Write;

//https://github.com/oxidecomputer/progenitor?tab=readme-ov-file#buildrs
fn main() {
    // let swagger_file = "swagger.json";
    // match reqwest::blocking::get("http://127.0.0.1:8000/api/swagger.json") {
    let swagger_file = "swagger.yaml";
    match reqwest::blocking::get("http://127.0.0.1:8000/api/swagger.yaml") {
        Ok(rsp) => {
            let mut file = std::fs::File::create(swagger_file).unwrap();
            file.write_all(&rsp.bytes().unwrap()).unwrap();
        }
        Err(_) => {} //ignore, continue with existing file
    }

    println!("cargo:rerun-if-changed={}", swagger_file);
    let file = std::fs::File::open(swagger_file).unwrap();
    // let spec = serde_json::from_reader(file).unwrap();
    let spec = serde_yml::from_reader(file).unwrap();
    let mut generator = progenitor::Generator::default();

    let tokens = generator.generate_tokens(&spec).unwrap();
    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    //https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    let mut out_file =
        std::path::Path::new(&(std::env::var("CARGO_MANIFEST_DIR").unwrap().to_string() + "/src"))
            .to_path_buf();
    out_file.push("client.rs");

    std::fs::write(out_file, content).unwrap();
}
