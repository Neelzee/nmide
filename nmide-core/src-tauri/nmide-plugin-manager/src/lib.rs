use anyhow::{Context, Result};
use libloading::{self, Library, Symbol};
use nmide_rust_ffi::{
    html::{chtml_location, Html},
    interface::functions::{Init, View},
    model::{Model, Msg},
    CModel, CMsg,
};
use safer_ffi::prelude::AsOut;
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
        let _manifest: Symbol<unsafe extern "C" fn() -> CModel> = unsafe {
            lib.get(b"manifest")
                .context("Failed loading plugin, no manifest")
        }?;
        let manifest = Model::from_c(unsafe { _manifest() })?;
        Ok(Self {
            id: Uuid::new_v4(),
            name: name.into(),
            lib,
            manifest,
        })
    }

    pub fn init(&self) -> Result<Model> {
        let _init: Symbol<Init> =
            unsafe { self.lib.get(b"init") }.context("Failed getting `init`")?;

        Model::from_c(unsafe { _init() })
    }

    pub fn view(&self, model: Model) -> Result<(String, Html)> {
        let _view: Symbol<View> =
            unsafe { self.lib.get(b"view") }.context("Failed getting `view`")?;

        chtml_location(unsafe { _view(model.to_c()?).as_out().as_mut_ptr() })
    }

    pub fn update(&self, msg: Msg, model: Model) -> Result<Model> {
        let _update: Symbol<unsafe extern "C" fn(CMsg, CModel) -> CModel> =
            unsafe { self.lib.get(b"update") }.context("Failed getting `update`")?;

        Model::from_c(unsafe { _update(msg.to_c()?, model.to_c()?) })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lib(&self) -> &Library {
        &self.lib
    }

    pub fn manifest(&self) -> &Model {
        &self.manifest
    }
}
