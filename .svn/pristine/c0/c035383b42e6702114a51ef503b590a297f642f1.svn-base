use crate::{Kind, Module, run_cmd};
use std::process::Command;

pub(crate) fn install(dist: String, mods: Vec<Module>) -> Vec<String> {
    let mut scripts = Vec::new();
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
        let mut copy_cmd = Command::new("cp");
        copy_cmd.arg("-r");
        copy_cmd.arg(&path);
        copy_cmd.arg(format!("{}/{}.{}", &dist, m.name, m.kind.as_ext()));
        run_cmd(copy_cmd);
        let script = format!(
            "<script src='./dist/external/{}.{}' type={}></script>",
            &m.name,
            m.kind.as_ext(),
            if m.kind == Kind::MJavaScript {
                "module"
            } else {
                "\"\""
            },
        );
        scripts.push(script);
    }
    scripts
}
