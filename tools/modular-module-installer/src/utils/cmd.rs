use anyhow::{Context, Result, anyhow};
use std::fmt::Display;
use tokio::process::Command;

#[derive(Debug)]
pub struct Cmd {
    cmd: String,
    args: Vec<String>,
}

impl Cmd {
    pub fn new<S: ToString>(cmd: S, args: Vec<S>) -> Self {
        Self {
            cmd: cmd.to_string(),
            args: args.into_iter().map(|x| x.to_string()).collect(),
        }
    }

    async fn exec(self) -> Result<i32> {
        Command::new(&self.cmd)
            .args(&self.args)
            .status()
            .await
            .context(format!("Failed executing cmd: {:?}", self))
            .and_then(|s| {
                s.code()
                    .ok_or(anyhow!("Failed getting ExitStatus: {:?}", self))
            })
    }

    pub async fn exec_or_prnt<S: Display>(self, s: S) -> bool {
        self.exec().await.map_or_else(
            |err| {
                eprintln!("{}: {}", s, err);
                false
            },
            |v| {
                if v == 0 {
                    true
                } else {
                    eprintln!("{}: Non-zero exitcode: {}", s, v);
                    false
                }
            },
        )
    }
}
