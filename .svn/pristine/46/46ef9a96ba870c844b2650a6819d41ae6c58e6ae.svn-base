use std::process::Command;

use crate::{Kind, Module, run_cmd};

pub(crate) fn install(modules: Vec<Module>, module_folder: String) {
    println!("JSM runtime installer");
    for m in modules {
        if !m.enabled {
            continue;
        }
        if matches!(m.kind, Kind::Css | Kind::Rust) {
            continue;
        }

        let name = m.name().to_string();

        let mut path = m.path.clone();

        if let Some(pm) = m.package_manager {
            println!("Building module: {:?}", &path);
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

        println!("Copying module: {}", name);
        let mut copy_cmd = Command::new("cp");
        copy_cmd.arg("-p");
        copy_cmd.arg(&path);
        copy_cmd.arg(format!("{}{name}.js", &module_folder));
        println!(
            "{} {}",
            copy_cmd.get_program().to_str().unwrap(),
            copy_cmd
                .get_args()
                .map(|a| a.to_str().unwrap())
                .collect::<Vec<_>>()
                .join(" ")
        );
        run_cmd(copy_cmd);
    }
}
