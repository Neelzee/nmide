use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out);
    let file = out_dir.join("..").join("..").join("..").join("..");
    let file = file
        .canonicalize()
        .unwrap_or_else(|err| panic!("Could not canonicalize {:?}, error: {:?}", &file, err));
    let file = file.join("module_reg.rs");
    if !file.exists() {
        let mut empty_file = File::create(&file).unwrap_or_else(|err| {
            panic!("File creation should succeed {:?}, error: ${:?}", file, err)
        });
        let buff =
            "pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {}".as_bytes();
        empty_file.write_all(buff).unwrap_or_else(|err| {
            panic!(
                "Writing to file should succeed {:?}, error: ${:?}",
                file, err
            )
        });
    }
    fs::copy(file.clone(), out_dir.join("module_reg.rs"))
        .unwrap_or_else(|err| panic!("File {:?} should exist, error: {:?}", file, err));
    tauri_build::build();
}
