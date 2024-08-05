extern crate cmake;

use cmake::Config;
use std::process::Command;

fn main() {
    let mut binding = Command::new("bindgen");
    let res = binding.arg("nmide.h").arg("-o").arg("src/bindings.rs");

    println!("{:?}", &res);

    #[cfg(feature = "test-lib")]
    build_with_lib();
}

fn build_with_lib() {
    let dst = Config::new("../../nmide-lib").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=nmide");
}
