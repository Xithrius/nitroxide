#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env, fs};

#[tauri::command]
fn home_folder_items() -> Vec<String> {
    let home = env::var("HOME").unwrap();

    let entries = fs::read_dir(home)
        .unwrap()
        .flat_map(|res| {
            res.map_or(None, |e| {
                Some(
                    e.path()
                        .to_str()
                        .unwrap()
                        .to_string()
                        .split('/')
                        .last()
                        .unwrap()
                        .to_string(),
                )
            })
        })
        .collect::<Vec<String>>();

    entries
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![home_folder_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
