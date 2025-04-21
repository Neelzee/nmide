use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use toml::Value;
use crate::{Kind, Module};

const MODULE_SEPARATOR: &'static str =
    "# ============================================================================ #";
pub(crate) fn install(modules: Vec<Module>, cargo: String, out: String) {
    let mut module_imports = Vec::new();
    let mut module_reg = Vec::new();
    let cargo_path = Path::new(&cargo).canonicalize().unwrap();

    let mut tbl = toml::Table::new();
    let mut mods = Vec::new();
    for m in modules {

        if !m.enabled || m.kind != Kind::Rust {
            continue;
        }

        let mut t = toml::Table::new();
        let module = m.name().to_string();
        t.insert(module.clone(), m.to_value());
        mods.push(t);
        module_imports.push(format!("extern crate {};", module));
        module_reg.push(format!(
            "modules.insert(\"{}\".to_string(), Box::new({}::ModuleBuilder.build()));",
            module, module
        ));
    }
    tbl.insert(
        "dependencies".to_string(),
        Value::try_from(mods).expect("Should be valid serialization")
    );

    let c = toml::to_string_pretty(&tbl)
        .and_then(|p| Ok(p.replace("[[dependencies]]\n", "")))
        .unwrap();

    let mut file = File::open(&cargo_path).expect("Cargo.toml file not found");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Cargo.toml file read error");

    if !buf.contains(MODULE_SEPARATOR) {
        buf.push_str(format!("\n{}" ,MODULE_SEPARATOR).as_str());
    }

    let mut final_buffer = String::new();

    for line in buf.lines() {
        final_buffer.push_str(format!("{}\n", line).as_str());
        if line == MODULE_SEPARATOR {
            break;
        }
    }

    final_buffer.push_str(&format!("\n{c}"));

    fs::write(cargo_path, final_buffer).unwrap();
    let p = Path::new(&out).join("module_reg.rs");
    let mut reg_file = File::create(p).unwrap();
    for import in &module_imports {
        writeln!(reg_file, "{}", import).unwrap();
    }

    writeln!(
        reg_file,
        "pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {{"
    )
        .unwrap();
    for registration in &module_reg {
        writeln!(reg_file, "    {}", registration).unwrap();
    }
    writeln!(reg_file, "}}").unwrap();
}
