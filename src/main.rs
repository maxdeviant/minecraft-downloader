mod manifest;

use std::io::Cursor;
use std::path::PathBuf;

use reqwest::Url;
use tokio::fs;

use crate::manifest::LauncherManifest;

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
