use std::sync::Mutex;

use once_cell::sync::Lazy;

pub static ROOT_FOLDER: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::new())
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

