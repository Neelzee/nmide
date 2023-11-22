use std::sync::Mutex;
use std::path::Path;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use walkdir::{WalkDir, DirEntry};

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
    File { name: String, path: String },
    Folder { name: String, path: String, contents: Vec<FolderOrFile> },
}


#[tauri::command]
pub async fn get_content_in_folder(root: String) -> FolderOrFile {
    match get_all_files_and_folders(&root) {
    Ok(res) => res,
    Err(_) => FolderOrFile::Folder { name: root.clone(), path: root, contents: Vec::new() },
    }
}

pub fn get_all_files_and_folders(root: &str) -> Result<FolderOrFile, Box<dyn std::error::Error>> {
    let root_path = Path::new(root);
    if !root_path.is_dir() {
        return Err("Root path is not a directory".into());
    }

    fn build_structure(entry: DirEntry) -> Result<FolderOrFile, Box<dyn std::error::Error>> {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();

        if path.is_dir() {
            let mut contents = Vec::new();
            for entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                contents.push(build_structure(entry));
            }
            Ok(FolderOrFile::Folder { name, path: path.to_string_lossy().into(), contents: contents.into_iter().filter_map(|p| p.ok()).collect() })
        } else {
            Ok(FolderOrFile::File { name, path: path.to_string_lossy().into() })
        }
    }

    let con: Vec<FolderOrFile> = WalkDir::new(root_path)
        .into_iter()
        .fold(Vec::new(), |mut acc, el| {
            if let Err(_) = el { return acc; }

            if let Ok(r) = build_structure(el.unwrap()) {
                acc.push(r);
            }
            acc
        });

    let mut root_name = root;

    if let Some(fm) = root_path.file_name() {
        if let Some(f) = fm.to_str() {
            root_name = f;
        }
    }

    Ok(FolderOrFile::Folder { name: root_name.to_owned(), path: root.to_owned(), contents: con })

}

