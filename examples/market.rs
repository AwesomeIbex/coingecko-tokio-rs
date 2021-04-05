use coingecko_tokio::{Client, MarketRequest};

#[tokio::main]
pub async fn main() {
    tokio::spawn(async {
        let http = reqwest::Client::new();

        let client = Client::new(http);

        let req = MarketRequest::new(
            String::from("usd"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        println!("{:#?}", client.markets(req).await);
    });
}
