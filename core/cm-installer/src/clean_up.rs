use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;

use crate::rs_installer::MODULE_SEPARATOR;

pub(crate) fn clean_up(index: String, cargo_path: String) {
    println!("Cleaning: {index}");
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(&index)
        .unwrap_or_else(|err| panic!("Could not open file: {:?}, due to error: {:?}", index, err));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read contents");
    let mut new_contents = String::new();
    let mut ignore = false;
    for line in contents.lines() {
        if line.contains("<!--MODULES-->") {
            ignore = !ignore;
            new_contents.push_str(format!("<!--MODULES-->\n").as_ref());
            continue;
        }
        if !ignore {
            new_contents.push_str(format!("{line}\n").as_ref());
        }
    }
    fs::write(index, new_contents).expect("Should be able to write to file");

    println!("Cleaning: {cargo_path}");
    let mut file = File::open(&cargo_path).expect("Cargo.toml file not found");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cargo.toml file read error");

    if !buf.contains(MODULE_SEPARATOR) {
        buf.push_str(format!("\n{}", MODULE_SEPARATOR).as_str());
    }

    let mut final_buffer = String::new();

    for line in buf.lines() {
        final_buffer.push_str(format!("{}\n", line).as_str());
        if line == MODULE_SEPARATOR {
            break;
        }
    }

    fs::write(cargo_path, final_buffer).unwrap();
}
