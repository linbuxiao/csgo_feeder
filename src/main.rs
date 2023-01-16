mod config;
mod fetcher;
mod model;
mod sender;

use config::load_config;
use fetcher::Fetcher;
use sender::{Sender, WorkWechatSender};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use log::info;
use pretty_env_logger;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let feeder_config = load_config();
    let fetcher_list: Vec<Fetcher> = feeder_config
        .feed_map
        .iter()
        .map(|(long_steam_id, name)| {
            Fetcher::new(
                feeder_config.my_steam_id.clone(),
                long_steam_id.to_string(),
                feeder_config.access_token.clone(),
                name.to_string(),
            )
        })
        .collect();
    for f in fetcher_list {
        let infos = f.run()?;
        let first_info = infos.get(0).unwrap().clone();
        info!("wait send info: {:#?}", first_info);
        let work_wechat_sender =
            WorkWechatSender::new(first_info, feeder_config.webhook_key.clone());
        work_wechat_sender.send();
        sleep(Duration::from_secs(2));
    }
    info!("game over");
    Ok(())
}
