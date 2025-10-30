use log::warn;
use std::process::Command;

pub(crate) const DIST_DIR: &str = env!("DIST_DIR");

pub(crate) const MODULE_SEPARATOR: &str =
    "# ============================================================================ #";

pub(crate) fn run_cmd(mut cmd: Command) {
    match cmd.status() {
        Ok(st) if st.success() => (),
        Ok(st) => warn!(
            "Got non zero exit code! {:?},\
                 when running command: {:?} {:?}",
            st,
            cmd.get_program(),
            cmd.get_args(),
        ),
        Err(err) => warn!(
            "Failed running command: {:?} {:?}, got error: {:?}",
            cmd.get_program(),
            cmd.get_args(),
            err,
        ),
    }
}
