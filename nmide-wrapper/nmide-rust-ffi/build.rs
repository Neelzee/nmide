use std::env;
use std::path::PathBuf;

fn main() {
    println!("Building");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-search=./html/");
    println!("cargo:rust-link-lib=./html/css_lib.o");
    println!("cargo:rust-link-lib=./html/html_lib.o");
    println!("cargo:rust-link-lib=./nmidelib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("nmidelib.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap_or_default());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
