use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use toml::Value;

const MODULE_SEPARATOR: &'static str =
    "# ============================================================================ #";
fn main() {
    let mut args = std::env::args();
    args.next();
    let mut conf = String::new();
    let mut out = String::new();
    let mut cargo = String::new();
    for a in args.into_iter() {
        if a.contains("--conf=") {
            conf = a.replace("--conf=", "");
            continue;
        }
        if a.contains("--out=") {
            out = a.replace("--out=", "");
            continue
        }
        if a.contains("--cargo=") {
            cargo = a.replace("--cargo=", "");
        }
    }
    let mut module_imports = Vec::new();
    let mut module_reg = Vec::new();
    let module_toml_path = Path::new(&conf);
    if module_toml_path.exists() {
        let module_content = fs::read_to_string(module_toml_path)
            .expect("Path should exist, and be of valid encoding.");
        let module_config: Value =
            toml::from_str(&module_content).expect("Module should be a valid TOML");
        let cargo_path = Path::new(&cargo);

        let mut tbl = toml::Table::new();
        let mut mods = Vec::new();
        if let Some(modules) = module_config.get("modules").and_then(|p| p.as_table()) {
            for (module, spec) in modules {
                if spec
                    .get("kind")
                    .is_some_and(|p| p.as_str().is_some_and(|k| k == "js"))
                {
                    continue;
                }
                if spec
                    .get("enabled")
                    .is_some_and(|p| p.as_bool().is_some_and(|k| k))
                {
                    continue;
                }
                let mut t = toml::Table::new();
                t.insert(module.clone(), spec.clone());
                mods.push(t);
                module_imports.push(format!("extern crate {};", module));
                module_reg.push(format!(
                    "modules.insert(\"{}\".to_string(), Box::new({}::ModuleBuilder.build()));",
                    module, module
                ));
            }
        }
        tbl.insert(
            "dependencies".to_string(),
            Value::try_from(mods).expect("Should be valid serialization")
        );

        let c = toml::to_string_pretty(&tbl).and_then(|p| Ok(p.replace("[[dependencies]]\n", ""))).unwrap();

        let mut file = File::open(cargo_path).expect("Cargo.toml file not found");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Cargo.toml file read error");

        let mut final_buffer = String::new();

        for line in buf.lines() {
            final_buffer.push_str(format!("{}\n", line).as_str());
            if line == MODULE_SEPARATOR {
                break;
            }
        }

        final_buffer.push_str(&format!("\n{c}"));

        fs::write(cargo_path, final_buffer).unwrap();

        let mut reg_file = fs::File::create(Path::new(&out).join("module_reg.rs")).unwrap();
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
}