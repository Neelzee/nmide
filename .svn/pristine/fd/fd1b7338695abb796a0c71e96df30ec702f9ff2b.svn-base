use crate::{Module, run_cmd};
use std::{fs, process::Command};

pub(crate) fn install(dist: String, mods: Vec<Module>) {
    let mut imports = Vec::new();
    for m in mods {
        if !m.enabled {
            continue;
        }
        if !m.kind.is_js() {
            continue;
        }

        let mut path = m.path.clone();

        println!("Installing module: {:?}", &path);

        if let Some(pm) = m.package_manager {
            let mut install_cmd = Command::new(pm.clone());
            install_cmd.current_dir(&path);
            install_cmd.arg("i");
            run_cmd(install_cmd);
            let mut build_cmd = Command::new(pm);
            build_cmd.current_dir(&path);
            build_cmd.arg("run");
            build_cmd.arg("build");
            run_cmd(build_cmd);
            path = path.join("build/index.js")
        }
        let import = format!("import '{}'", path.as_os_str().to_str().unwrap());
        imports.push(import);
    }

    let s: String = imports.join("\n");

    fs::write(format!("{dist}/modules.js"), s).unwrap();
}
