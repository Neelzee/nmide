use anyhow::{Context, Result};
use libloading::{self, Library, Symbol};
use nmide_std_lib::{html::rhtml::RHtml, map::rmap::RMap, msg::rmsg::RMsg};
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

    pub fn init(&self) -> Result<RMap> {
        let _init: Symbol<unsafe extern "C" fn() -> RMap> =
            unsafe { self.lib.get(b"init") }.context("Failed getting Rust `init`")?;
        Ok(unsafe { _init() })
    }

    pub fn view(&self, model: RMap) -> Result<RHtml> {
        let _view: Symbol<unsafe extern "C" fn(RMap) -> RHtml> =
            unsafe { self.lib.get(b"view") }.context("Failed getting Rust `view`")?;

        Ok(unsafe { _view(model) })
    }

    pub fn update(&self, msg: RMsg, model: RMap) -> Result<RMap> {
        let _update: Symbol<unsafe extern "C" fn(RMsg, RMap) -> RMap> =
            unsafe { self.lib.get(b"update") }.context("Failed getting Rust `update`")?;

        Ok(unsafe { _update(msg, model) })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
