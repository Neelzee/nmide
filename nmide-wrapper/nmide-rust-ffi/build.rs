use anyhow::{anyhow, Context, Result};
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let nmide_lib_path: PathBuf = PathBuf::from("../../nmide-lib")
        .canonicalize()
        .context("cannot canonicalize path")?;

    let header_path = nmide_lib_path.join("nmide.h");
    let header_path_str = header_path.to_str().context("path is not valid String")?;
    let lib_release_path = nmide_lib_path.join("release");

    if !std::process::Command::new("mkdir")
        .arg("-p")
        .arg("../../nmide-lib/release")
        .output()
        .context("could not spawn `mkdir`")?
        .status
        .success()
    {
        return Err(anyhow!("Could not create release"));
    }

    if !std::process::Command::new("make")
        .arg("-C")
        .arg(&lib_release_path)
        .output()
        .context("could not spawn `make`")?
        .status
        .success()
    {
        return Err(anyhow!("Could not compile library"));
    }

    println!(
        "cargo:rustc-link-search={}",
        lib_release_path
            .to_str()
            .context("lib path is not valid String")?
    );

    println!("cargo:rustc-link-lib=nmide");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .context("Unable to generate bindings")?;

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .context("Couldn't write bindings!")
}
