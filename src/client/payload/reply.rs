use reqwest::Client as Reqwest;
use serde_json::{Value, json};

pub struct Reply<'a> {
    pub token: &'a String,
    pub message_id: u64,
    pub req: &'a Reqwest,
    pub with_parse: Vec<String>,
}

impl<'a> Reply<'a> {
    pub async fn send_message(&self, channel_id: u64, content: &str) -> Value {
        let res: Value = self.req
            .post(format!("https://discord.com/api/v10/channels/{channel_id}/messages"))
            .json(&json!({
                "content": content,
                "message_reference": { "message_id": self.message_id },
                "allowed_mentions": {
                    "parse": self.with_parse
                }
            }))
            .header("Authorization", format!("Bot {}", self.token))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("A")
            .json()
            .await
            .expect("J");
        res
    }

    pub fn parse(&self, parse: Vec<&str>) -> Self {
        Self {
            token: &self.token,
            message_id: self.message_id,
            req: &self.req,
            with_parse: parse.iter().map(|s| s.to_string()).collect(),
        }
    }
}
