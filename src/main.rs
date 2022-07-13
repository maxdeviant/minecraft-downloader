use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LauncherManifest {
    pub files: HashMap<String, LauncherEntry>,
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

    println!("{:#?}", manifest);

    Ok(())
}
