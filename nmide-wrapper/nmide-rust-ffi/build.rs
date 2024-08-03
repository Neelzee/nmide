extern crate cmake;

use cmake::Config;

fn main() {
    #[cfg(feature = "test_lib")]
    build_with_lib();
}

fn build_with_lib() {
    let dst = Config::new("../../nmide-lib").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=nmide");
}
