use std::io::Write;
use std::{fs, path::Path};
use toml::Value;

fn main() {
    let mut module_imports = Vec::new();
    let mut module_reg = Vec::new();
    let module_toml_path = Path::new("../modules/Modules.toml");
    if module_toml_path.exists() {
        let module_content = fs::read_to_string(module_toml_path)
            .expect("Path should exist, and be of valid encoding.");
        let module_config: Value =
            toml::from_str(&module_content).expect("Module should be a valid TOML");
        let cargo_path = Path::new("Cargo.toml");
        let mut cargo: Value = toml::from_str(&fs::read_to_string(cargo_path).unwrap()).unwrap();

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
                cargo
                    .as_table_mut()
                    .unwrap()
                    .get_mut("dependencies")
                    .unwrap()
                    .as_table_mut()
                    .unwrap()
                    .insert(module.clone(), spec.clone());
                module_imports.push(format!("extern crate {};", module));
                module_reg.push(format!(
                    "modules.insert(\"{}\".to_string(), Box::new({}::ModuleBuilder.build()));",
                    module, module
                ));
            }
        }

        fs::write(cargo_path, toml::to_string(&cargo).expect("The mapping between Module.toml [modules] and Cargo.toml [dependencies] should be one-to-one")).unwrap();

        let out_dir = env::var("OUT_DIR").unwrap();
        let mut reg_file = fs::File::create(Path::new(&out_dir).join("module_reg.rs")).unwrap();
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

    #[cfg(feature = "ide")]
    tauri_build::build();
}
