use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidUserResp {
    pub status_code: i64,
    pub error_message: String,
    pub data: Vec<ValidUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidUser {
    pub group: String,
    pub data: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameInfo {
    pub name: String,
    pub start_time: String,
    pub is_win: bool,
    pub score_1: u8,
    pub score_2: u8,
    pub rating: f64,
    pub map_name: String,
    pub kill: u8,
    pub death: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub map_name: String,
    pub team: i8,
    pub win_team: i8,
    pub score1: u8,
    pub score2: u8,
    pub pw_rating: f64,
    pub start_time: String,
    pub kill: u8,
    pub death: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchReq {
    pub my_steam_id: String,
    pub game_type_str: String,
    pub steam_id: String,
    pub access_token: String,
    pub data_source: u8,
    pub page_size: u8,
    pub csgo_season_id: String,
    pub pvp_type: i8,
    pub page: u8,
}
