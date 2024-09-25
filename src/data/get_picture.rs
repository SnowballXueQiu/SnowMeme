use crate::data::MemeMeta;
use futures::StreamExt;
use std::fs::exists;
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download_picture(meta: &MemeMeta) {
    download_file(&meta.url, meta.get_picture_path()).await.unwrap();
}

async fn download_file(url: &str, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    fs::create_dir_all(&path.parent().unwrap()).await.unwrap();
    if exists(&path)? {
        return Ok(());
    }
    let mut file = File::create(&path).await?;

    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        file.write_all(&item?).await?;
    }

    Ok(())
}
