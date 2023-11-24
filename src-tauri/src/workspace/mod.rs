use std::{sync::{Mutex, MutexGuard, PoisonError}, fs};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use walkdir::{WalkDir, DirEntry};
use std::path::{Path, PathBuf};
use std::io::Read;


struct Workspace {
    pub root: String,
    pub open_files: Vec<File>
}

impl Workspace {
    pub fn default() -> Workspace {
        Workspace {
            root: String::new(),
            open_files: Vec::new(),
        }
    }
}

static WORKSPACE: Lazy<Mutex<Workspace>> = Lazy::new(|| Mutex::new(Workspace::default()));

#[derive(Deserialize, Serialize, Debug)]
pub struct File {
    name: String,
    path: String
}

/**
 * Opens the given file, and returns the content
 */
#[tauri::command]
pub fn open_file(path: String) -> Option<String> {
    if let Ok(mut workspace) = WORKSPACE.lock() {

        if workspace.open_files.iter().filter(|f| f.path == path).collect::<Vec<&File>>().len() != 0 {
            return None;
        }

        fs::File::open(path.clone())
            .and_then(|mut file| {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                let name;

                if let Some(pn) = Path::new(&path.clone()).file_name().and_then(|f| f.to_str()) {
                    name = pn.to_owned();
                } else {
                    name = path.clone();
                }

                workspace.open_files.push(File{ name, path });

                Ok(contents)
            })
            .ok();

    }
    None
}

#[tauri::command]
pub fn get_root_folder() -> Option<String> {
    let lock_result = WORKSPACE.lock();

    match lock_result {
        Ok(guard) => Some(guard.root.clone()), 
        Err(_) => None,
    }
}

#[tauri::command]
pub fn set_root_folder(new_root: String) -> Option<()> {
    if let Ok(mut lock) = WORKSPACE.lock() {
   
        lock.root = new_root;

        return Some(());
    }
    None
}


#[derive(Deserialize, Serialize, Debug)]
pub enum FolderOrFile {
    File(File),
    Folder {
        name: String,
        path: String,
        contents: Vec<FolderOrFile>
    },
}


#[tauri::command]
pub fn get_content_in_folder(root: String) -> FolderOrFile {
    match get_all_files_and_folders(&root) {
    Ok(res) => res,
    Err(_) => FolderOrFile::Folder { name: root.clone(), path: root, contents: Vec::new() },
    }
}
fn build_structure(path: &Path, depth: i32) -> Result<FolderOrFile, Box<dyn std::error::Error>> {
    let name = path.file_name().ok_or("No filename")?.to_string_lossy().into_owned();
    let path_str = path.to_string_lossy().into_owned();

    if path.is_dir() {
        let mut contents = Vec::new();
        for entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            if entry_path == path {
                continue; // Skip the root of the current iteration
            }

            if depth == 0 {
                return Ok(FolderOrFile::Folder { name, path: path_str, contents });
            }
            contents.push(build_structure(entry_path, depth - 1)?);
        }
        Ok(FolderOrFile::Folder { name, path: path_str, contents })
    } else {
        Ok(FolderOrFile::File(File{ name, path: path_str }))
    }
}

pub fn get_all_files_and_folders(root: &str) -> Result<FolderOrFile, Box<dyn std::error::Error>> {
    let root_path = PathBuf::from(root);
    build_structure(&root_path, 1)
}