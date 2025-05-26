use std::process::Command;

use crate::{Kind, Module, run_cmd};

pub(crate) fn install(modules: Vec<Module>, module_folder: String) {
    println!("RSM runtime installer");
    for m in modules {
        if !m.enabled {
            continue;
        }
        if m.kind != Kind::Rust {
            continue;
        }

        let mut path = m.path.clone();

        println!("Building module: {:?}", &path);

        let mut build_cmd = Command::new("cargo");
        build_cmd.current_dir(&path);
        build_cmd.arg("build");
        build_cmd.arg("--release");
        run_cmd(build_cmd);
        // TODO: Ensure this works for windows with "dll"
        path = path.join(format!("target/release/lib{}.so", m.name.replace("-", "_"),));
        let mut copy_cmd = Command::new("cp");
        copy_cmd.arg("-p");
        copy_cmd.arg(&path);
        copy_cmd.arg(&module_folder);
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
