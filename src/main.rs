use cordius::client::Client;
use cordius::client::Payload;

#[tokio::main]
async fn main() {
    Client::new("", true).await;
    let payload = Payload::new();
    payload.send_message(1396644937927360572, "<@&1403257534604775455>").await;
    payload.parse(vec![]).send_message(1396644937927360572, "<@&1403257534604775455>").await;
}
