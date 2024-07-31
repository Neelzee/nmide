use anyhow::{Context, Result};
use libloading::{self, Library, Symbol};
use nmide_rust_ffi::{html::Html, CHtml};
use std::ffi::OsStr;

pub enum NmluginType {
    Worker,
    Manager,
}

pub struct Nmlugin {
    name: String,
    p_type: NmluginType,
    lib: Library,
}

impl Nmlugin {
    pub fn new<P>(name: impl Into<String>, p_type: NmluginType, path: P) -> Result<Self>
    where
        P: AsRef<OsStr>,
    {
        Ok(Self {
            name: name.into(),
            p_type,
            lib: unsafe { Library::new(&path) }.context("Failed loading pluging")?,
        })
    }

    pub fn view(&self) -> Result<Html> {
        let _view: Symbol<unsafe extern "C" fn() -> CHtml> =
            unsafe { self.lib.get(b"view") }.context("Failed getting `view`")?;

        Ok(Html::from_c(unsafe { _view() }))
    }
}
