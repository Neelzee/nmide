use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    std_types::{ROption, RVec},
};
use core_std_lib::{
    html::rhtml::RHtml,
    map::rmap::{RKeyPair, RMap, RValue},
    msg::rmsg::RMsg,
    NmideStandardLibraryRef, NmideStdLib,
};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[export_root_module]
pub fn get_library() -> NmideStandardLibraryRef {
    NmideStdLib { init, view, update }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn init() -> RMap {
    RMap::new()
}

#[sabi_extern_fn]
pub fn view(_: &RMap) -> RHtml {
    RHtml::Frag(RVec::new(), RVec::new())
}

#[sabi_extern_fn]
pub fn update(msg: &RMsg, model: &RMap) -> RMap {
    match msg {
        m if m.is_msg("fsa-read") => {
            if let Some(file) = m.val().get_value().str() {
                match read_file(file, model) {
                    Ok(data) => RMap::new().insert(
                        &format!("fsa-read-{}", file.to_string()),
                        RValue::new_str(data),
                    ),
                    Err(err) => RMap::new().insert("fsa-error", error(&err, msg)),
                }
            } else {
                RMap::new().insert("fsa-error", error("Expected msg to contain string", msg))
            }
        }
        m if m.is_msg("fsa-write") => {
            if let Some(obj) = m.val().get_value().obj() {
                match write_file(obj, model) {
                    Ok(file) => {
                        RMap::new().insert(&format!("fsa-write-{}", file), RValue::new_bool(true))
                    }
                    Err(err) => RMap::new().insert("fsa-error", error(&err, msg)),
                }
            } else {
                RMap::new().insert("fsa-error", error("Expected msg to contain string", msg))
            }
        }
        m if m.is_msg("fsa-dir") => {
            if let Some(file) = m.val().get_value().str() {
                match walk_dir(file, model) {
                    Ok(data) => RMap::new().insert(
                        &format!("fsa-dir-{}", file.to_string()),
                        RValue::new_list(data),
                    ),
                    Err(err) => RMap::new().insert("fsa-error", error(&err, msg)),
                }
            } else {
                RMap::new().insert("fsa-error", error("Expected msg to contain string", msg))
            }
        }
        _ => RMap::new(),
    }
}

fn read_file(key: &str, model: &RMap) -> Result<String, String> {
    match model.lookup(key) {
        ROption::RSome(val) => {
            let path: PathBuf = val
                .str()
                .ok_or("Could not match GUID to file")?
                .to_string()
                .into();
            let mut file = File::open(path).map_err(|err| format!("{err:?}"))?;
            let mut buf = String::new();
            file.read_to_string(&mut buf)
                .map_err(|err| format!("{err:?}"))?;
            Ok(buf)
        }
        ROption::RNone => Err(format!("Could not find: {key} in model")),
    }
}

fn write_file(obj: &RVec<RKeyPair>, model: &RMap) -> Result<String, String> {
    let key: &str = obj
        .iter()
        .find(|kp| kp.cmp_key("fsa-file"))
        .ok_or("Could not find `fsa-file` in msg-obj")?
        .val()
        .str()
        .ok_or("Expected `fsa-file` in msg-obj to be string")?;
    let buf: &str = obj
        .iter()
        .find(|kp| kp.cmp_key("fsa-content"))
        .ok_or("Could not find `fsa-content` in msg-obj")?
        .val()
        .str()
        .ok_or("Expected `fsa-content` in msg-obj to be string")?;
    match model.lookup(key) {
        ROption::RSome(val) => {
            let path: PathBuf = val
                .str()
                .ok_or("Could not match GUID to file")?
                .to_string()
                .into();
            let mut file = File::open(path).map_err(|err| format!("{err:?}"))?;
            file.write_all(buf.as_bytes())
                .map_err(|err| format!("{err:?}"))?;
            Ok(key.to_string())
        }
        ROption::RNone => Err(format!("Could not find: {key} in model")),
    }
}

fn walk_dir(key: &str, model: &RMap) -> Result<Vec<String>, String> {
    match model.lookup(key) {
        ROption::RSome(val) => {
            let path: PathBuf = val
                .str()
                .ok_or("Could not match GUID to file")?
                .to_string()
                .into();
            Ok(walk(&path, true)?
                .into_iter()
                .map(|f| f.to_string_lossy().to_string())
                .collect())
        }
        ROption::RNone => Err(format!("Could not find: {key} in model")),
    }
}

fn walk(pth: &Path, ignore_hidden: bool) -> Result<Vec<PathBuf>, String> {
    if ignore_hidden && pth.starts_with(".") {
        return Ok(Vec::new());
    }
    if pth.is_dir() {
        Ok(pth
            .read_dir()
            .map_err(|err| format!("{:?}", err))?
            .flat_map(|f| {
                f.map(|d| walk(&d.path(), ignore_hidden).unwrap_or_default())
                    .unwrap_or_default()
            })
            .collect())
    } else {
        Ok(vec![pth.to_path_buf()])
    }
}

fn error(info: &str, msg: &RMsg) -> RValue {
    let vec: Vec<(&str, RValue)> = vec![
        ("info", info.into()),
        ("msg", msg.get_msg().to_string().into()),
        ("value", msg.val().get_value().clone()),
    ];
    RValue::new_obj(vec)
}
