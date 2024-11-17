use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    rvec, sabi_extern_fn,
    std_types::{ROption, RString, RVec},
};
use anyhow::Result;
use nmide_std_lib::{
    attr::rattr::{RAttr, RAttrKind, RAttrUnion},
    html::rhtml::RHtml,
    map::rmap::{RMap, RValue},
    msg::rmsg::{RMsg, RMsgKind, RMsgUnion},
    NmideStandardLibraryRef, NmideStdLib,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, read_dir, File},
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
        .insert("info-module-path", false)
        .insert("info-module-file-path", "")
        .insert("info-module-graph", "")
}

#[sabi_extern_fn]
pub fn view(module: &RMap) -> RHtml {
    RHtml::Div(
        rvec![
            RHtml::Input(
                RVec::new(),
                rvec![RAttr::new_emit_input(
                    RString::from_str("info-module-update-input").unwrap_or_default()
                )],
            ),
            RHtml::Button(
                rvec![RHtml::text(
                    RString::from_str("Find File").unwrap_or_default()
                )],
                rvec![RAttr::new_click(RMsg::new(
                    RMsgKind::Msg,
                    RMsgUnion::new(
                        RString::from_str("info-module-find-file").unwrap_or_default(),
                        RValue::new_str(
                            module
                                .lookup("info-module-file-path")
                                .into_option()
                                .and_then(|s| s.str())
                                .map(|s| s.to_string())
                                .unwrap_or_default()
                        )
                    )
                ))],
            ),
        ],
        RVec::new(),
    )
}

#[derive(Serialize, Deserialize)]
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

impl Module {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let mut split = buffer.split_terminator("package");
        split.next();
        let mut content_name = split.next().unwrap_or_default().split_whitespace();
        let name = content_name.next().unwrap_or_default().trim().to_string();

        let mut split = buffer.split_terminator("imports");
        split.next(); // everything before imports
                      // Everything after
        let content = split.next().unwrap_or_default();
        let mut content_split = content.split_terminator(";");
        // Should be all imports
        let imports_line = content_split.next().unwrap_or_default();
        let imports: Vec<String> = imports_line
            .split_whitespace()
            .map(|i| i.to_string())
            .collect();

        Ok(Self { name, imports })
    }
}

#[sabi_extern_fn]
pub fn update(msg: &RMsg, _: &RMap) -> RMap {
    if msg.is_msg("info-module-update-input") {
        return RMap::new().insert(
            "info-module-file-path",
            msg.val()
                .get_value()
                .str()
                .map(|s| ManuallyDrop::into_inner(s.clone()))
                .map(|s| s.to_string())
                .unwrap_or_default(),
        );
    }

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

    RMap::new().insert(
        "info-module-graph",
        serde_json::to_string::<Data>(&to_data(to_tree(get_modules(path).unwrap_or_default())))
            .unwrap_or_default(),
    )
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

fn to_tree(modules: Vec<Module>) -> HashMap<String, Vec<String>> {
    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    for module in modules {
        if tree.contains_key(&module.name) {
            let mut old_imports: Vec<String> = tree.get(&module.name).unwrap().clone();
            for import in module.imports {
                if !old_imports.contains(&import) {
                    old_imports.push(import);
                }
            }
            tree.insert(module.name, old_imports);
        } else {
            tree.insert(module.name, module.imports);
        }
    }
    tree
}

fn to_data(tree: HashMap<String, Vec<String>>) -> Data {
    let mut nodes: Vec<Node> = Vec::new();
    let mut links: Vec<Link> = Vec::new();
    for (id, targets) in tree {
        let node = Node { id: id.clone() };
        nodes.push(node);
        for target in targets {
            links.push(Link {
                source: id.clone(),
                target,
            });
        }
    }

    Data { links, nodes }
}
