//https://github.com/oxidecomputer/progenitor?tab=readme-ov-file#buildrs
fn main() {
    let src = "../api/swagger.json";
    println!("cargo:rerun-if-changed={}", src);
    let file = std::fs::File::open(src).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
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
