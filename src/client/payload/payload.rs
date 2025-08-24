use reqwest::Client as Reqwest;
use serde_json::{Value, json};
use crate::{client::Reply};
use dotenvy::dotenv;
use std::env;

#[cfg(feature = "payload")]
pub struct Payload {
    /// The token of your client
    token: String,
    /// The HTTP client
    req: Reqwest,
    /// The parse object, determines what the bot should ping
    with_parse: Vec<String>,
}

#[cfg(feature = "payload")]
impl Payload {

    /// Initializes a new Payload
    ///
    /// # Returns
    /// Payload
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            // Gets the token from the saved .env file
            token: env::var("DISCORD_TOKEN").expect("Could not find token in .env").to_string(),
            req: Reqwest::new(),
            with_parse: vec!["users".to_string(), "roles".to_string(), "everyone".to_string()],
        }
    }

    pub async fn send_message(&self, channel_id: u64, content: &str) -> Value {
        let res: Value = self.req
            .post(format!("https://discord.com/api/v10/channels/{channel_id}/messages"))
            .json(&json!({ 
                "content": content,
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

    pub async fn edit_message(&self, channel_id: u64, message_id: u64, new_content: &str) {
        let res = self.req
            .patch(format!("https://discord.com/api/v10/channels/{channel_id}/messages/{message_id}"))
            .json(&json!({ "content": new_content }))
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

    pub fn reply(&self, message_id: u64) -> Reply {
        Reply { 
            token: &self.token,
            message_id, 
            req: &self.req,
            with_parse: vec!["users".to_string(), "roles".to_string(), "everyone".to_string()],
        }
    }

    pub fn parse(&self, parse: Vec<&str>) -> Self { 
        Self {
            token: (*self.token).to_string(),
            req: self.req.clone(),
            with_parse: parse.iter().map(|s| s.to_string()).collect(),
        }
    }
}
