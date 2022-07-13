use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LauncherManifest {
    pub files: HashMap<String, LauncherEntry>,
}

impl LauncherManifest {
    pub fn directories(&self) -> Vec<String> {
        self.files
            .iter()
            .filter_map(|(key, entry)| match entry {
                LauncherEntry::Directory => Some(key.clone()),
                LauncherEntry::File(_) => None,
            })
            .collect()
    }

    pub fn files(self) -> Vec<(String, LauncherFile)> {
        self.files
            .into_iter()
            .filter_map(|(key, entry)| match entry {
                LauncherEntry::File(file) => Some((key, file)),
                LauncherEntry::Directory => None,
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LauncherEntry {
    Directory,
    File(LauncherFile),
}

#[derive(Debug, Deserialize)]
pub struct LauncherFile {
    pub downloads: Downloads,
    pub executable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub raw: File,
    pub lzma: Option<File>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub sha1: String,
    pub size: i32,
    pub url: String,
}
