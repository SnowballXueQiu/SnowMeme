use crate::data::MemeMeta;
use crate::task::{post_greeting, post_meme};
use ::config::ext::{CommandLineConfigurationBuilderExtensions, ConfigurationBinder, EnvironmentVariablesExtensions, FileSourceBuilderExtensions, JsonConfigurationExtensions};
use config::XBotConfig;
use ::config::{ConfigurationBuilder, DefaultConfigurationBuilder};
use lazy_static::lazy_static;
use log::info;
use rcron::{Job, JobScheduler};
use tweety_rs::TweetyClient;

mod config;
mod data;
mod post;
mod task;

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

    info!("Creating tasks");

    let mut sched = JobScheduler::new();
    //                  sec   min   hour           day of month   month   day of week   year
    sched.add(Job::new("0     0     *              *              *       *             *".parse().unwrap(), || {
        info!("Posting meme");
        tokio::spawn(task::post_meme());
    }));

    sched.add(Job::new("0     30    7,12,15,18,22  *              *       *             *".parse().unwrap(), || {
        info!("Posting greeting");
        tokio::spawn(task::post_greeting());
    }));

    loop {
        sched.tick();
        tokio::time::sleep(sched.time_till_next_job()).await;
    }
}
