use std::env;

fn main() {
    // Instruct Cargo to link the shared library
    println!("cargo:rustc-link-lib=add");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Instruct Cargo to look for the shared library in the current directory
    println!("cargo:rustc-link-search={}", dir);
}
