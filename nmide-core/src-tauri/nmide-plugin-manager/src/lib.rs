use anyhow::{Context, Result};
use libloading::{self, Library, Symbol};
use nmide_std_lib::{
    html::Html,
    interface::rfunctions::{RInit, RUpdate, RView},
    map::Map,
    msg::Msg,
};
use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug)]
pub struct Nmlugin {
    lib: Library,
    path: PathBuf,
}

impl Nmlugin {
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<OsStr>,
    {
        Ok(Self {
            lib: unsafe { Library::new(&path) }.context("Failed loading plugin")?,
            path: PathBuf::from(&path),
        })
    }

    pub fn init(&self) -> Result<Map> {
        let _init: Symbol<RInit> =
            unsafe { self.lib.get(b"init") }.context("Failed getting Rust `init`")?;
        Ok(unsafe { _init() })
    }

    pub fn view(&self, model: Map) -> Result<Html> {
        let _view: Symbol<RView> =
            unsafe { self.lib.get(b"view") }.context("Failed getting Rust `view`")?;

        Ok(unsafe { _view(model) })
    }

    pub fn update(&self, msg: Msg, model: Map) -> Result<Map> {
        let _update: Symbol<RUpdate> =
            unsafe { self.lib.get(b"update") }.context("Failed getting Rust `update`")?;

        Ok(unsafe { _update(msg, model) })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
