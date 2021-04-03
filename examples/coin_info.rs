#[tokio::main]
pub async fn main() {
    tokio::spawn(async {
        let http = reqwest::Client::new();

        let client = coingecko::Client::new(http);

        println!("{:#?}", client.coin_info("algorand").await);
    }).await;
}
