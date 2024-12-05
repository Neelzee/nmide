use anyhow::{anyhow, Result};
use std::{path::PathBuf, process::Command};

fn main() -> Result<()> {
    let path = PathBuf::from("../../nmide-plugin").canonicalize()?;
    for dir in path.read_dir()? {
        match dir {
            Ok(d) => {
                if !d.path().is_dir() {
                    continue;
                }
                match Command::new("cargo")
                    .arg("build")
                    .arg("--release")
                    .current_dir(d.path())
                    .status()
                {
                    Ok(res) => println!("{:?}: {:?}", d.path(), res),
                    Err(err) => {
                        return Err(anyhow!(
                            "Failed to build plugin at path: `{d:?}`, err: {err:?}"
                        ))
                    }
                }
            }
            Err(err) => eprintln!("Failed to build plugin at path: `{err:?}`"),
        }
    }

    Ok(())
}
