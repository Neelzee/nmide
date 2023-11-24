// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::workspace::{get_root_folder, set_root_folder, get_content_in_folder, open_file};

mod workspace;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_root_folder, // Gets root folder
      set_root_folder, // Sets root folder
      get_content_in_folder, // Gets content from the given folder
      open_file, // Opens a file and return the content from it
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
