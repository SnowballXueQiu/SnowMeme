use ::config::{
    ext::{
        CommandLineConfigurationBuilderExtensions, ConfigurationBinder,
        EnvironmentVariablesExtensions, FileSourceBuilderExtensions, JsonConfigurationExtensions
    },
    ConfigurationBuilder, DefaultConfigurationBuilder,
};
use config::XBotConfig;
use lazy_static::lazy_static;
use log::info;
use tweety_rs::TweetyClient;

mod config;

lazy_static! {
    static ref XBOT_CONFIG: XBotConfig = {
        let xbot_config = DefaultConfigurationBuilder::new()
            .add_json_file("config/xbot.json".is().optional())
            .add_env_vars()
            .add_command_line()
            .build()
            .unwrap();

        xbot_config.reify()
    };
    static ref TWEETY_CLIENT: TweetyClient = {
        TweetyClient::new(
            &XBOT_CONFIG.consumer_key,
            &XBOT_CONFIG.access_token,
            &XBOT_CONFIG.consumer_key_secret,
            &XBOT_CONFIG.access_token_secret,
        )
    };
}

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log.json", Default::default()).unwrap();

    // let me = TWEETY_CLIENT
    //     .get_user_me(None)
    //     .await
    //     .expect("Cannot get self");

    // info!(
    //     "Logged in as {}",
    //     me.get("data")
    //         .unwrap()
    //         .get("name")
    //         .unwrap()
    //         .as_str()
    //         .unwrap()
    // );

    info!("111");
}
