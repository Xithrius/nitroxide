#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{cmp::Ordering, env, fs};

use serde::Serialize;

#[derive(Serialize, PartialEq, PartialOrd, Eq)]
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

impl Ord for DirItem {
    // Final result should be dirs, hidden dirs, files, hidden files.
    // All are sorted by name in each section.
    // TODO: Hidden file sorting by name is out of order.
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .is_directory
            .cmp(&self.is_directory)
            .then(self.is_hidden.cmp(&other.is_hidden))
            .then(self.name.cmp(&other.name))
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

    entries.sort_by(|a, b| a.cmp(&b));

    entries
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![folder_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
