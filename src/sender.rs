use serde_json::json;

use crate::model::GameInfo;

pub trait Sender {
    fn send(&self);
}

pub struct WorkWechatSender {
    content: GameInfo,
    send_key: String,
}

impl WorkWechatSender {
    pub fn new(content: GameInfo, send_key: String) -> Self {
        Self { content, send_key }
    }
}

impl Sender for WorkWechatSender {
    fn send(&self) {
        let client = reqwest::blocking::Client::new();
        let webhook_url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key={}",
            self.send_key.clone()
        );
        let info = &self.content;
        client.post(webhook_url).body(json!({
                "msgtype": "text", 
                "text": {
                    "content": format!(
                        "报！！{}游戏结束\n开始时间: {}\n比分: [{}:{}]\n地图: [{}]\n{}KD: [{}/{}], rating: {}",
                        info.name,
                        info.start_time,
                        info.score_1,
                        info.score_2,
                        info.map_name,
                        info.name,
                        info.kill,
                        info.death,
                        info.rating,
                    )
                }
            }).to_string()).send().unwrap();
    }
}
