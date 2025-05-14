use std::fs;
use std::path::Path;

fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out);
    let file = out_dir.join("..").join("..").join("..").join("..");
    let file = file
        .canonicalize()
        .expect(&format!("Could not canonicalize {:?}", &file));
    let file = file.join("module_reg.rs");
    fs::copy(file.clone(), out_dir.join("module_reg.rs"))
        .expect(&format!("File {:?} should exist", file));
    tauri_build::build();
}
