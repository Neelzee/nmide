use super::{Module, run_cmd};
use std::{fs, path::PathBuf, process::Command};

pub(crate) fn install(dist: &str, mods: Vec<Module>) {
    println!("JSM compile-time installer");
    let mut imports = Vec::new();
    for m in mods {
        if !m.enabled {
            continue;
        }
        if !m.kind.is_js() {
            continue;
        }

        let mut path = m.path.clone();

        if let Some(pm) = m.package_manager {
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

    let pth: PathBuf = format!("{dist}/modules.js").into();
    if !pth.exists() {
        fs::create_dir_all(dist).expect("Dir creation should succeed");
    }
    fs::write(&pth, s)
        .inspect_err(|err| panic!("Could not write to file: {pth:?}, due to error: {err:?}"))
        .unwrap();
}
