use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;

use reqwest::Url;
use serde::Deserialize;
use tokio::fs;

#[derive(Debug, Deserialize)]
struct LauncherManifest {
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
enum LauncherEntry {
    Directory,
    File(LauncherFile),
}

#[derive(Debug, Deserialize)]
struct LauncherFile {
    pub downloads: Downloads,
    pub executable: bool,
}

#[derive(Debug, Deserialize)]
struct Downloads {
    pub raw: File,
    pub lzma: Option<File>,
}

#[derive(Debug, Deserialize)]
struct File {
    pub sha1: String,
    pub size: i32,
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = reqwest::get("https://launchermeta.mojang.com/v1/packages/688bc2598db8ce144841701ad2b0b11a1bf8035f/manifest.json")
        .await?
        .json::<LauncherManifest>()
        .await?;

    let download_dir = "test";

    for launcher_directory in manifest.directories() {
        let mut directory = PathBuf::new();
        directory.push(download_dir);
        directory.push(launcher_directory);

        fs::create_dir_all(directory).await?;
    }

    for (launcher_path, launcher_file) in manifest.files() {
        let url = Url::parse(&launcher_file.downloads.raw.url)?;

        let response = reqwest::get(url).await?;

        let mut path = PathBuf::new();
        path.push(download_dir);
        path.push(launcher_path);

        let mut file = fs::File::create(path).await?;
        let mut content = Cursor::new(response.bytes().await?);

        tokio::io::copy(&mut content, &mut file).await?;
    }

    Ok(())
}
