use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;
use crate::{Kind, Module};
use std::os::unix::fs::FileExt;
use regex::Regex;

const MODULE_SEP: &'static str = "<!--MODULES-->";

pub(crate) fn install(index: String, dist: String, mods: Vec<Module>) {
    let mut scripts = Vec::new();
    for m in mods {
        if !m.enabled {
            continue;
        }
        if m.kind == Kind::Rust {
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
    let mut file = OpenOptions::new().read(true).write(true).open(&index)
        .expect(format!("Could not open file: {:?}", index).as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read contents");
    let regex = Regex::new(r#"<!--MODULES-->(\s*|.*)*<!--MODULES-->"#).unwrap();
    let mut new_scripts = format!("{}\n", MODULE_SEP);
    new_scripts.push_str(scripts.join("\n").as_str());
    new_scripts.push_str(format!("\n{}", MODULE_SEP).as_str());
    let new_content = regex.replace(&contents, new_scripts).to_string();
    file.write_at(&new_content.as_bytes(), 0).expect("Should be able to write to file");
}

fn run_cmd(mut cmd: Command) {
    match cmd.status() {
        Ok(st) if st.success() => (),
        Ok(st) => eprintln!(
            "Got non zero exit code! {:?},\
                 when running command: {:?} {:?}",
            st,
            cmd.get_program(),
            cmd.get_args(),
        ),
        Err(err) => eprintln!(
            "Failed running command: {:?} {:?}, got error: {:?}",
            cmd.get_program(),
            cmd.get_args(),
            err,
        )
    }
}