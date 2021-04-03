#[macro_use]
extern crate derive_setters;
#[macro_use]
extern crate fomat_macros;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

mod coin_info;
mod simple_price;
mod utils;

use const_format::concatcp;


pub use crate::coin_info::*;
pub use crate::simple_price::*;

const API: &str = "https://api.coingecko.com/api/v3/";

pub struct Client {
    http: reqwest::Client,
}

impl Client {
    pub fn new(http: reqwest::Client) -> Self {
        Self { http }
    }

    /// Check if CoinGecko is reachable
    pub async fn ping(&self) -> Result<Ping, Error> {
        const PING: &str = concatcp!(crate::API, "/ping");

        utils::get_json(&self.http, PING).await
    }

    /// Fetches the current price of any cryptocurrencies in any other supported currencies you need.
    pub async fn simple_price(&self, req: SimplePriceReq) -> Result<SimplePrices, Error> {
        const SIMPLE: &str = concatcp!(crate::API, "/simple/price");

        let uri = fomat!((SIMPLE) "?" (req.query()));

        utils::get_json(&self.http, &uri).await
    }

    /// Fetches detailed information about a particular coin by its ID.
    pub async fn coin_info(&self, coin: &str) -> Result<CoinInfo, Error> {
        const COINS: &str = concatcp!(crate::API, "/coins");

        let uri = fomat!((COINS) "/" (coin));

        utils::get_json(&self.http, &uri).await
    }

    /// Fetches a list of coins supported by CoinGecko
    pub async fn coins_list(&self) -> Result<Vec<Coin>, Error> {
        const COINS_LIST: &str = concatcp!(crate::API, "/coins/list");

        utils::get_json(&self.http, COINS_LIST).await
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error")]
    Http(#[from] reqwest::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Deserialization error")]
    Deserialization(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Ping {
    gecko_says: String,
}
