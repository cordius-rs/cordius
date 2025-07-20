use cordius::Client;


#[tokio::main]
async fn main() {

    let client = Client::new("...");
    let res = client.application.current().await;
    println!("{:#?}", res.unwrap());
}
