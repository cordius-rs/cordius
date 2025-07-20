use tokio_tungstenite::{connect_async, tungstenite::{Result}, WebSocketStream};
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;

use crate::api::Endpoint;
use crate::channel::Channel;
use crate::application::Application;

pub struct Client<'a> {
  pub token: Option<&'a str>,
  ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
  pub channel: Channel<'a>,
  pub application: Application<'a>,
}

impl<'a> Client<'a> {
  pub fn new<T: Into<Option<&'a str>> + Copy>(token: T) -> Self {
    Client {
      token: token.into(),
      ws: None,
      channel: Channel::new(token),
      application: Application::new(token),
    }
  }

  // Unfinished cause fuck websocket
  pub async fn login(mut self) -> Result<Self, String> {
    if self.token.is_none() || self.token.unwrap_or("").is_empty() {
      return Err(String::from("Expected token got nothing!"));
    }
    let (socket, _) = connect_async(format!("{}", Endpoint::WSS))
      .await
      .map_err(|e| format!("Failed to connect with the reason {e}!"))?;
    self.ws = Some(socket);
    println!("ok");
    Ok(self)
  }
}
