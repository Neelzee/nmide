use std::process::Command;

use crate::{Kind, Module, run_cmd};

pub(crate) fn install(modules: Vec<Module>, module_folder: String) {
    for m in modules {
        if !m.enabled {
            continue;
        }
        if m.kind != Kind::RustRt {
            continue;
        }

        let mut path = m.path.clone();

        println!("Building module: {:?}", &path);

        let mut build_cmd = Command::new("cargo");
        build_cmd.current_dir(&path);
        build_cmd.arg("build");
        build_cmd.arg("--release");
        run_cmd(build_cmd);
        path = path.join(format!(
            "target/release/lib{}.{}",
            m.name.replace("-", "_"),
            m.kind.as_ext()
        ));
        let mut copy_cmd = Command::new("cp");
        copy_cmd.arg("-p");
        copy_cmd.arg(&path);
        copy_cmd.arg(&module_folder);
        println!(
            "{} {}",
            copy_cmd.get_program().to_str().unwrap(),
            copy_cmd
                .get_args()
                .into_iter()
                .map(|a| a.to_str().unwrap())
                .collect::<Vec<_>>()
                .join(" ")
        );
        run_cmd(copy_cmd);
    }
}
