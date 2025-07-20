use reqwest::{Client, Error};
use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::api::Endpoint;

pub struct Channel<'a> {
  token: Option<&'a str>,
  req: Client,
}

impl<'a> Channel<'a> {
  pub fn new<T: Into<Option<&'a str>>>(token: T) -> Self {
    Channel {
      token: token.into(),
      req: Client::new(),
    }
  }

  pub async fn get_channel(&self, channel_id: u64) -> Result<Value, Error> {
    let res = self.req
      .get(format!("{}{}", Endpoint::BASE_URL, Endpoint::CHANNEL(channel_id)))
      .header("Authorization", format!("Bot {}", self.token.unwrap()))
      .send()
      .await?;
    let json: Value = res.json().await?;
    Ok(json)
  }
  
  pub async fn delete_channel(&self, channel_id: u64) -> Result <Value> {
    let res = self.req
      .delete(format!("{}{}", Endpoint::BASE_URL, Endpoint::CHANNEL(channel_id)))
      .header("Authorization", format!("Bot {}", self.token.unwrap()))
      .send()
      .await?;
    let stat = res.status();
    let json: Value = res.json().await?;
    if !stat.is_success() {
      match json.get("code").and_then(|r| r.as_u64()) {
        Some(50013) => return Err(anyhow!("The bot lacks permissions to delete the channel {channel_id}!")),
        Some(10003) => return Err(anyhow!("Channel {channel_id} either doesn't exist or the bot does not have access to it.")),
        Some(code) => return Err(anyhow!("Unhandled error, code {code}: {}\n Report to devs with the code and message!", json["message"])),
        None => return Err(anyhow!("Unknown error")),
      }
    }
    Ok(json)
  }
}
