use anyhow::{anyhow, Context, Result};
use either::Either;
use libloading::{self, Library, Symbol};
use nmide_std_lib::{
    html::Html,
    interface::rfunctions::{RInit, RManifest, RUpdate, RView},
    map::{value::Value, Map},
    msg::Msg,
};
use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug)]
pub struct Nmlugin {
    lib: Library,
    init_fn: Option<Either<(), ()>>,
    view_fn: Option<Either<(), ()>>,
    update_fn: Option<Either<(), ()>>,
    manifest: Map,
    path: PathBuf,
}

impl Nmlugin {
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<OsStr>,
    {
        let lib = unsafe { Library::new(&path) }.context("Failed loading plugin")?;
        let _manifest: Symbol<RManifest> = unsafe {
            lib.get(b"manifest")
                .context("Failed loading plugin, no manifest")
        }?;
        let manifest = unsafe { _manifest() };
        let mut init_fn = None;
        let mut view_fn = None;
        let mut update_fn = None;
        let is_rust = manifest
            .lookup("nmide-plugin-type")
            .is_some_and(|a| match a {
                Value::String(x) if x.to_lowercase() == "rust" => true,
                _ => false,
            });
        match manifest.lookup("nmide-functions") {
            Some(Value::List(funcs)) => {
                for func in funcs {
                    match func {
                        Value::String(x) => match x.to_lowercase().as_str() {
                            "init" => {
                                if is_rust {
                                    init_fn = Some(Either::Right(()));
                                } else {
                                    init_fn = Some(Either::Left(()));
                                }
                            }
                            "view" => {
                                if is_rust {
                                    view_fn = Some(Either::Right(()));
                                } else {
                                    view_fn = Some(Either::Left(()));
                                }
                            }
                            "update" => {
                                if is_rust {
                                    update_fn = Some(Either::Right(()));
                                } else {
                                    update_fn = Some(Either::Left(()));
                                }
                            }
                            _ => continue,
                        },
                        _ => continue,
                    }
                }
            }
            _ => todo!(),
        }
        Ok(Self {
            lib,
            init_fn,
            view_fn,
            update_fn,
            manifest,
            path: PathBuf::from(&path),
        })
    }

    pub fn init(&self) -> Result<Map> {
        match self.init_fn {
            Some(Either::Left(_)) => {
                unimplemented!()
            }
            Some(Either::Right(_)) => {
                let _init: Symbol<RInit> =
                    unsafe { self.lib.get(b"init") }.context("Failed getting Rust `init`")?;
                Ok(unsafe { _init() })
            }
            _ => Err(anyhow!("No `init` in plugin")),
        }
    }

    pub fn view(&self, model: Map) -> Result<Html> {
        match self.view_fn {
            Some(Either::Left(_)) => {
                unimplemented!()
            }
            Some(Either::Right(_)) => {
                let _view: Symbol<RView> =
                    unsafe { self.lib.get(b"view") }.context("Failed getting Rust `view`")?;

                Ok(unsafe { _view(model) })
            }
            _ => Err(anyhow!("No `view` in plugin")),
        }
    }

    pub fn update(&self, msg: Msg, model: Map) -> Result<Map> {
        match self.update_fn {
            Some(Either::Left(_)) => {
                unimplemented!()
            }
            Some(Either::Right(_)) => {
                let _update: Symbol<RUpdate> =
                    unsafe { self.lib.get(b"update") }.context("Failed getting Rust `update`")?;

                Ok(unsafe { _update(msg, model) })
            }
            _ => Err(anyhow!("No `update` in plugin")),
        }
    }

    pub fn manifest(&self) -> &Map {
        &self.manifest
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
