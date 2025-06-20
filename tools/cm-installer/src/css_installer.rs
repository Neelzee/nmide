use crate::{Kind, Module, run_cmd};
use std::{path::PathBuf, process::Command};

pub(crate) fn install(dist: String, mods: Vec<Module>) -> Vec<String> {
    println!("CSS installer");
    let mut styles = Vec::new();
    for m in mods {
        if !m.enabled {
            continue;
        }
        if m.kind != Kind::Css {
            continue;
        }

        let path = m.path.clone();

        let mut copy_cmd = Command::new("cp");
        copy_cmd.arg(path.join(format!("{}.{}", m.name, m.kind.as_ext())));
        copy_cmd.arg(format!("{}/{}.{}", &dist, m.name, m.kind.as_ext()));
        run_cmd(copy_cmd);
        let script = format!(
            r#"<link rel="stylesheet" href="{}/{}.{}" type="text/css"/>"#,
            (PathBuf::from(&dist)
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap_or_default()),
            m.name,
            m.kind.as_ext(),
        );
        styles.push(script);
    }
    styles
}
