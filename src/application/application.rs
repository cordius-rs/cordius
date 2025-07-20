use reqwest::Client;
use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::api::Endpoint;

pub struct Application<'a> {
  token: Option<&'a str>,
  req: Client,
}

impl<'a> Application<'a> {
  pub fn new<T: Into<Option<&'a str>>>(token: T) -> Self {
    Application {
      token: token.into(),
      req: Client::new(),
    }
  }

  pub async fn current(&self) -> Result<Value> {
    let res = self.req
      .get(format!("{}{}", Endpoint::BASE_URL, Endpoint::CURRENT_APPLICATION))
      .header("Authorization", format!("Bot {}", self.token.unwrap()))
      .send()
      .await?;
      
    let stat = res.status();
    let json: Value = res.json().await?;
    if !stat.is_success() {
      match json.get("code").and_then(|r| r.as_u64()) {
        Some(code) => return Err(anyhow!("Unhandled error, code {code}: {}\n Report to devs with the code and message!", json["message"])),
        None => return Err(anyhow!("Unknown error")),
      }
    }
    Ok(json)
  }
}
