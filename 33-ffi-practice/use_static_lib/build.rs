use std::path::PathBuf;

fn main() {
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../target"));
    println!("cargo:rustc-link-search={}/debug", target.display());
}
