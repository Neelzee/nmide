use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use walkdir::{WalkDir, DirEntry};
use std::path::{Path, PathBuf};

pub static ROOT_FOLDER: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::from("."))
});

#[tauri::command]
pub async fn get_root_folder() -> Option<String> {
    let lock_result = ROOT_FOLDER.lock();

    match lock_result {
        Ok(guard) => Some(guard.clone()), 
        Err(_) => None,
    }
}

#[tauri::command]
pub async fn set_root_folder(new_root: String) -> Option<()> {
    if let Ok(mut lock) = ROOT_FOLDER.lock() {
   
        *lock = new_root;

        return Some(());
    }
    None
}


#[derive(Deserialize, Serialize, Debug)]
pub enum FolderOrFile {
    File {
        name: String,
        path: String
    },
    Folder {
        name: String,
        path: String,
        contents: Vec<FolderOrFile>
    },
}


#[tauri::command]
pub async fn get_content_in_folder(root: String) -> FolderOrFile {
    match get_all_files_and_folders(&root) {
    Ok(res) => res,
    Err(_) => FolderOrFile::Folder { name: root.clone(), path: root, contents: Vec::new() },
    }
}
fn build_structure(path: &Path) -> Result<FolderOrFile, Box<dyn std::error::Error>> {
    let name = path.file_name().ok_or("No filename")?.to_string_lossy().into_owned();
    let path_str = path.to_string_lossy().into_owned();

    if path.is_dir() {
        let mut contents = Vec::new();
        for entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            if entry_path == path {
                continue; // Skip the root of the current iteration
            }
            contents.push(build_structure(entry_path)?);
        }
        Ok(FolderOrFile::Folder { name, path: path_str, contents })
    } else {
        Ok(FolderOrFile::File { name, path: path_str })
    }
}

pub fn get_all_files_and_folders(root: &str) -> Result<FolderOrFile, Box<dyn std::error::Error>> {
    let root_path = PathBuf::from(root);
    build_structure(&root_path)
}