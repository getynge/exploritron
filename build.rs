use std::env;
use std::path::PathBuf;

fn main() {
    let mdir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target = "render-stdout/target/debug";
    println!("cargo:rustc-link-search=dylib={}", mdir.join(target).display())
}