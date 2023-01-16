use crate::model::{GameInfo, MatchInfo, MatchReq, ValidUserResp};
use log::{debug, info};
use md5::{Digest, Md5};
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::error::Error;

pub struct Fetcher {
    base_steam_id: String,
    long_steam_id: String,
    access_token: String,
    name: String,
}

impl Fetcher {
    pub fn new(
        base_steam_id: String,
        long_steam_id: String,
        access_token: String,
        name: String,
    ) -> Self {
        Self {
            base_steam_id,
            long_steam_id,
            access_token,
            name,
        }
    }

    fn build_payload(&self) -> MatchReq {
        MatchReq {
            my_steam_id: self.base_steam_id.clone(),
            game_type_str: "2".to_string(),
            steam_id: self.long_steam_id.clone(),
            access_token: self.access_token.clone(),
            data_source: 3,
            page_size: 10,
            csgo_season_id: "S10".to_string(),
            pvp_type: -1,
            page: 1,
        }
    }

    fn get_sign(&self) -> String {
        let salt = String::from("1f3192e58723aed15b8e8a9dc8e760861f3192e58723aed15b8e8a9dc8e76086");
        let mut hasher = Md5::new();
        let payload_str = serde_json::to_string(&self.build_payload()).unwrap();
        let data = payload_str + &salt;
        hasher.update(data.as_bytes());
        let hash = hasher.finalize();
        base16ct::lower::encode_string(&hash)
    }

    pub fn run(&self) -> Result<Vec<GameInfo>, Box<dyn Error>> {
        info!(
            "fetcher start run: access_token: {}, steam_id: {}",
            self.access_token, self.long_steam_id
        );
        let base_url = "https://api.wmpvp.com/api/v2/home/validUser?";
        let client = reqwest::blocking::Client::new();
        let headers_map: HashMap<String, String>  = [
            ("Host".to_string(), "api.wmpvp.com".to_string()),
            ("Accept".to_string(), "application/json, text/plain, */*".to_string()),
            ("X-Requested-With".to_string(), "XMLHttpRequest".to_string()),
            ("Accept-Language".to_string(), "en-US,en;q=0.9".to_string()),
            ("platform".to_string(), "h5_ios".to_string()),
            ("Accept-Encoding".to_string(), "gzip, deflate, br".to_string()),
            ("Origin".to_string(), "https://news.wmpvp.com".to_string()),
            ("Content-Length".to_string(), "210".to_string()),
            ("User-Agent".to_string(), "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148- EsportsApp Version=2.3.30".to_string()),
            ("Referer".to_string(), "https://news.wmpvp.com".to_string()),
            ("Connection".to_string(), "keep-alive".to_string()),
            ("Content-Type".to_string(), "application/json;charset=utf-8".to_string()),
        ].iter().cloned().collect();
        let headers: HeaderMap = (&headers_map).try_into().expect("valid headers");
        let raw_resp_builder = client
            .post(format!("{}sign={}", base_url, self.get_sign()))
            .json(&self.build_payload())
            .headers(headers);
        let raw_resp = raw_resp_builder.send()?;
        let resp = raw_resp.json::<ValidUserResp>().expect("json parse failed");
        debug!("get api resp: {:#?}", resp);
        let match_infos: Vec<MatchInfo> =
            serde_json::from_value(resp.data[2].data.clone()).unwrap();
        let infos: Vec<GameInfo> = match_infos
            .iter()
            .map(|match_info| {
                let mut is_win = false;
                if match_info.win_team == match_info.team {
                    is_win = true;
                }
                let mut score_1 = match_info.score1;
                let mut score_2 = match_info.score2;
                if match_info.team == 2 {
                    score_1 = match_info.score2;
                    score_2 = match_info.score1;
                }
                GameInfo {
                    name: self.name.clone(),
                    start_time: match_info.start_time.clone(),
                    is_win,
                    score_1,
                    score_2,
                    rating: match_info.pw_rating,
                    map_name: match_info.map_name.clone(),
                    kill: match_info.kill,
                    death: match_info.death,
                }
            })
            .collect();
        Ok(infos)
    }
}
