use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct XBotConfig {
    pub consumer_key: String,
    pub access_token: String,
    pub consumer_key_secret: String,
    pub access_token_secret: String,
}