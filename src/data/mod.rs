use log::debug;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod fetch_meta;
mod get_picture;
pub mod greeting;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemeMeta {
    pub post_link: String,
    pub subreddit: String,
    pub title: String,
    pub url: String,
    pub nsfw: bool,
    pub spoiler: bool,
    pub author: String,
    pub ups: i64,
    pub preview: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitokotoMeta {
    pub id: i64,
    pub uuid: String,
    pub hitokoto: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub from: String,
    pub creator: String,
    #[serde(rename = "creator_uid")]
    pub creator_uid: i64,
    pub reviewer: i64,
    #[serde(rename = "commit_from")]
    pub commit_from: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub length: i64,
}


impl MemeMeta {
    pub async fn new() -> Self {
        fetch_meta::fetch_meme_meta().await
    }

    pub fn get_picture_path(&self) -> PathBuf {
        let filename = self.url.split('/').last().unwrap();
        Path::new("cache").join(&filename[..2]).join(filename)
    }

    pub async fn download_picture(&self) {
        debug!("Downloading picture from {}", self.url);
        get_picture::download_picture(self).await;
    }
}

impl HitokotoMeta {
    pub async fn new() -> Self {
        fetch_meta::fetch_hitokoto_meta().await
    }
}
