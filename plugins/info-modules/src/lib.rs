use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::RVec,
};
use anyhow::{Context, Result};
use nmide_std_lib::{
    html::rhtml::RHtml, map::rmap::RMap, msg::rmsg::RMsg, NmideStandardLibraryRef, NmideStdLib,
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{read_dir, File},
    io::Read,
    mem::ManuallyDrop,
    path::PathBuf,
    str::FromStr,
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

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
struct Node {
    id: String,
}

#[derive(Serialize, Deserialize)]
struct Link {
    source: String,
    target: String,
}

#[derive(Serialize, Deserialize, Default)]
struct Data {
    links: Vec<Link>,
    nodes: Vec<Node>,
}

struct Module {
    name: String,
    imports: Vec<String>,
}

static IMPORTS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"imports\s+([^\n,]+,\s*)*[^;\n]+;").unwrap());

static PACKAGE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^package (.*)$").unwrap());

impl Module {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let imports_raw = IMPORTS_RE
            .find_iter(&buffer)
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        let imports_raw = imports_raw.replace("imports", "");
        let imports_raw = imports_raw.replace(",", "");
        let imports_raw = imports_raw.replace(";", "");
        let imports = imports_raw
            .split_whitespace()
            .filter(|p| !p.contains("//"))
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let name_raw = PACKAGE_RE
            .find_iter(&buffer)
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        let name = name_raw.replace("package", "").trim().to_string();

        Ok(Self { name, imports })
    }
}

#[sabi_extern_fn]
pub fn update(msg: &RMsg, _: &RMap) -> RMap {
    if !msg.is_msg("info-module-find-file") {
        return RMap::new();
    }

    let path = msg
        .val()
        .get_value()
        .str()
        .map(|s| ManuallyDrop::into_inner(s.clone()))
        .map(|s| s.to_string())
        .unwrap_or_default();
    let path = PathBuf::from_str(&path).unwrap_or_default();

    let result = run(path).unwrap_or_default();

    RMap::new().insert("info-module-graph", result)
}

fn run(path: PathBuf) -> Result<String> {
    let dirs = read_dir(path)?;
    let modules = dirs
        .filter_map(|p| p.map(|pl| pl.path()).ok())
        .filter(|p| p.is_dir())
        .filter(|p| !p.ends_with(".svn"))
        .flat_map(|p| get_modules(p).unwrap_or_default())
        .collect::<Vec<_>>();

    let mut node_set = HashSet::new();
    let mut links = Vec::new();

    for module in modules {
        let source = Node { id: module.name };
        if node_set.contains(&source) {
            continue;
        }
        for id in module.imports {
            let target = Node { id };
            links.push(Link {
                source: source.id.clone(),
                target: target.id.clone(),
            });
            node_set.insert(target);
        }
        node_set.insert(source);
    }

    let nodes = node_set.into_iter().collect();

    serde_json::to_string::<Data>(&Data { nodes, links })
        .context("Failed serializing Data to string")
}

fn get_modules(path: PathBuf) -> Result<Vec<Module>> {
    let mut dirs: Vec<_> = read_dir(path)?.collect();
    let mut modules = Vec::new();
    while let Some(dir) = dirs.pop() {
        if dir.is_err() {
            continue;
        };
        let dir = dir.unwrap();
        if dir.path().to_str().unwrap_or_default().eq(".svn") {
            continue;
        }
        if dir.path().is_dir() {
            dirs.push(Ok(dir));
            continue;
        }
        if dir.path().to_str().unwrap_or_default().eq("1.mg") {
            continue;
        }
        modules.push(Module::new(dir.path())?);
    }

    Ok(modules)
}
