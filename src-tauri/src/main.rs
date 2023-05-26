#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env, fs};

use serde::Serialize;

#[derive(Serialize)]
struct DirItem {
    name: String,
    is_directory: bool,
    is_hidden: bool,
}

impl DirItem {
    fn new(name: String, is_directory: bool, is_hidden: bool) -> Self {
        Self {
            name,
            is_directory,
            is_hidden,
        }
    }
}

#[tauri::command]
fn folder_items(context: Option<&str>) -> Vec<DirItem> {
    let ctx = if let Some(c) = context {
        c.to_string()
    } else {
        env::var("HOME").unwrap()
    };

    let mut entries = fs::read_dir(ctx)
        .unwrap()
        .flat_map(|res| {
            res.map_or(None, |e| {
                let item = e.path();

                let name = item
                    .to_str()
                    .unwrap()
                    .split('/')
                    .last()
                    .unwrap()
                    .to_string();

                Some(DirItem::new(
                    name.to_string(),
                    item.is_dir(),
                    name.starts_with("."),
                ))
            })
        })
        .collect::<Vec<DirItem>>();

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    entries
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![folder_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
