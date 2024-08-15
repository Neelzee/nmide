use anyhow::{Context, Result};
use libloading::{self, Library, Symbol};
use nmide_rust_ffi::{
    html::Html,
    interface::rfunctions::{RInit, RManifest, RUpdate, RView},
    model::{Model, Msg},
};
use std::ffi::OsStr;
use uuid::Uuid;

pub struct Nmlugin {
    id: Uuid,
    name: String,
    lib: Library,
    manifest: Model,
}

impl Nmlugin {
    pub fn new<P>(name: impl Into<String>, path: P) -> Result<Self>
    where
        P: AsRef<OsStr>,
    {
        let lib = unsafe { Library::new(&path) }.context("Failed loading plugin")?;
        let _manifest: Symbol<RManifest> = unsafe {
            lib.get(b"manifest")
                .context("Failed loading plugin, no manifest")
        }?;
        let manifest = unsafe { _manifest() };
        Ok(Self {
            id: Uuid::new_v4(),
            name: name.into(),
            lib,
            manifest,
        })
    }

    pub fn init(&self) -> Result<Model> {
        let _init: Symbol<RInit> =
            unsafe { self.lib.get(b"init") }.context("Failed getting `init`")?;

        Ok(unsafe { _init() })
    }

    pub fn view(&self, model: Model) -> Result<Html> {
        let _view: Symbol<RView> =
            unsafe { self.lib.get(b"view") }.context("Failed getting `view`")?;

        Ok(unsafe { _view(model) })
    }

    pub fn update(&self, msg: Msg, model: Model) -> Result<Model> {
        let _update: Symbol<RUpdate> =
            unsafe { self.lib.get(b"update") }.context("Failed getting `update`")?;

        Ok(unsafe { _update(msg, model) })
    }

    pub fn manifest(&self) -> &Model {
        &self.manifest
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
