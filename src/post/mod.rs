use tweety_rs::types::tweet::{Media, PostTweetParams};
use crate::data::MemeMeta;
use crate::TWEETY_CLIENT;

impl MemeMeta {
    async fn upload_meme(&self) -> Option<String> {
        let a = TWEETY_CLIENT.upload_file(&self.get_picture_path()).await;
        a.ok().map(|x| x.to_string())
    }

    pub async fn generate_media_param(&self) -> PostTweetParams {
        let media = self.upload_meme().await.unwrap();

        PostTweetParams {
            direct_message_deep_link: None,
            for_super_followers_only: None,
            geo: None,
            media: Some(Media {
                media_ids: vec![media].into(),
                tagged_user_ids: None,
            }),
            poll: None,
            quote_tweet_id: None,
            reply: None,
            reply_settings: None,
        }
    }
}
