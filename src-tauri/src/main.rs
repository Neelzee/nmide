// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::workspace::{get_root_folder, set_root_folder};

mod workspace;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_root_folder,
      set_root_folder
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
