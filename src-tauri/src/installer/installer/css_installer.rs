use super::{Kind, Module, run_cmd};
use std::{path::PathBuf, process::Command};

pub(crate) fn install(dist: &str, mods: Vec<Module>) -> Vec<String> {
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

        let css_path = env!("CSS_PATH");

        let mut copy_cmd = Command::new("cp");
        copy_cmd.arg(path.join(format!("{}.{}", m.name, m.kind.as_ext())));
        copy_cmd.arg(format!("{}/{}.{}", &dist, m.name, m.kind.as_ext()));
        run_cmd(copy_cmd);
        let script = format!(
            r#"<link rel="stylesheet" href="{}/{}.{}" type="text/css"/>"#,
            if !css_path.is_empty() {
                css_path.to_string()
            } else {
                PathBuf::from(&dist)
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap_or_default()
                    .to_string()
            },
            m.name,
            m.kind.as_ext(),
        );
        styles.push(script);
    }
    styles
}
