use std::collections::HashMap;
use std::process::exit;

use log::{error, info};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct FeederConfig {
    pub my_steam_id: String,
    pub access_token: String,
    pub webhook_key: String,
    pub feed_map: HashMap<String, String>,
}

impl ::std::default::Default for FeederConfig {
    fn default() -> Self {
        Self {
            my_steam_id: Default::default(),
            access_token: Default::default(),
            webhook_key: Default::default(),
            feed_map: Default::default(),
        }
    }
}

pub fn load_config() -> FeederConfig {
    let config: FeederConfig = confy::load("csgo_feeder", None).expect("load config failed");
    info!("load config success: {:#?}", config);
    if config.access_token == "".to_string() {
        error!("you are init empty config, need to fill the access_token (and something else) and run this toy again");
        exit(0);
    }
    config
}
