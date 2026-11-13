use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("third/cJSON.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cjson_bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new().file("third/cJSON.c").compile("cjson");
}
