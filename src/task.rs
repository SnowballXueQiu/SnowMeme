use crate::data::MemeMeta;
use crate::TWEETY_CLIENT;

pub async fn post_meme() {
    let meme = MemeMeta::new().await;
    meme.download_picture().await;
    let media_param = meme.generate_media_param().await;

    let message = format!("{}\nOriginal post: {}", meme.title, meme.post_link);

    TWEETY_CLIENT.post_tweet(message.as_str(), Some(media_param)).await.unwrap();
}

pub async fn post_greeting() {
    let greeting = crate::data::greeting::get_greeting();
    let hitokoto = crate::data::HitokotoMeta::new().await;

    let message = format!("{}\n一言: {} (hitokoto.cn posted by {})",
                          greeting, hitokoto.hitokoto, hitokoto.creator);

    TWEETY_CLIENT.post_tweet(message.as_str(), None).await.unwrap();
}
