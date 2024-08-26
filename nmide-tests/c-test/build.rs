use std::path::PathBuf;

fn main() {
    let lib_dir = PathBuf::from("../nmide-lib/release");

    // Tell Cargo to tell rustc to link the library
    println!("cargo:rustc-link-lib=static=nmide");

    // Tell Cargo to tell rustc where to find the library
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}
