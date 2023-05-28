use std::cmp::Ordering;

use serde::Serialize;

#[derive(Serialize, PartialEq, PartialOrd, Eq)]
pub struct DirItem {
    path: String,
    name: String,
    is_directory: bool,
    is_hidden: bool,
}

impl DirItem {
    pub fn new(path: String, name: String, is_directory: bool, is_hidden: bool) -> Self {
        Self {
            path,
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
