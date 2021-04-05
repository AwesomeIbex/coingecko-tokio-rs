#[tokio::main]
pub async fn main() {
    tokio::spawn(async {
        let http = reqwest::Client::new();

        let client = coingecko_tokio::Client::new(http);

        println!("{:#?}", client.ping().await);
    });
}
