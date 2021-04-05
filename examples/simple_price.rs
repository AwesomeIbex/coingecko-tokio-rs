use coingecko_tokio::{Client, SimplePriceReq};

#[tokio::main]
pub async fn main() {
    tokio::spawn(async {
        let http = reqwest::Client::new();

        let client = Client::new(http);

        let req = SimplePriceReq::new("ethereum,algorand,tezos".into(), "usd".into())
            .include_market_cap()
            .include_24hr_vol()
            .include_24hr_change()
            .include_last_updated_at();

        println!("{:#?}", client.simple_price(req).await);
    });
}
