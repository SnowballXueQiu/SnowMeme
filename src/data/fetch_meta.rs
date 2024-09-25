use crate::data::{HitokotoMeta, MemeMeta};
use lazy_static::lazy_static;
use rand::Rng;
use rand::rngs::OsRng;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}
static HITOKOTO_CATEGORY: [&str; 4] = ["d", "e", "i", "j"];

pub async fn fetch_meme_meta() -> MemeMeta {
    let res = CLIENT.get("https://meme-api.com/gimme")
        .send()
        .await
        .unwrap();
    res.json().await.unwrap()
}

pub async fn fetch_hitokoto_meta() -> HitokotoMeta {
    let mut rng = OsRng;
    let random_number: u32 = rng.gen_range(0..4);

    let res = CLIENT.get(format!("https://v1.hitokoto.cn?c={}", HITOKOTO_CATEGORY[random_number as usize]))
        .send()
        .await
        .unwrap();
    res.json().await.unwrap()
}
