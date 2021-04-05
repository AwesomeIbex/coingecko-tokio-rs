use std::fmt::Display;
use serde::{Serialize, Deserialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    #[serde(rename = "current_price")]
    pub current_price: f64,
    #[serde(rename = "market_cap")]
    pub market_cap: i64,
    #[serde(rename = "market_cap_rank")]
    pub market_cap_rank: i64,
    #[serde(rename = "fully_diluted_valuation")]
    pub fully_diluted_valuation: Option<i64>,
    #[serde(rename = "total_volume")]
    pub total_volume: f64,
    #[serde(rename = "high_24h")]
    pub high24_h: f64,
    #[serde(rename = "low_24h")]
    pub low24_h: f64,
    #[serde(rename = "price_change_24h")]
    pub price_change24_h: f64,
    #[serde(rename = "price_change_percentage_24h")]
    pub price_change_percentage24_h: f64,
    #[serde(rename = "market_cap_change_24h")]
    pub market_cap_change24_h: f64,
    #[serde(rename = "market_cap_change_percentage_24h")]
    pub market_cap_change_percentage24_h: f64,
    #[serde(rename = "circulating_supply")]
    pub circulating_supply: f64,
    #[serde(rename = "total_supply")]
    pub total_supply: Option<f64>,
    #[serde(rename = "max_supply")]
    pub max_supply: Option<f64>,
    pub ath: f64,
    #[serde(rename = "ath_change_percentage")]
    pub ath_change_percentage: f64,
    #[serde(rename = "ath_date")]
    pub ath_date: String,
    pub atl: f64,
    #[serde(rename = "atl_change_percentage")]
    pub atl_change_percentage: f64,
    #[serde(rename = "atl_date")]
    pub atl_date: String,
    pub roi: Option<Roi>,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roi {
    pub times: f64,
    pub currency: String,
    pub percentage: f64,
}

#[derive(Default, Setters, Clone)]
pub struct MarketRequest {
    /// ids of currency pairs, comma-separated
    #[setters(skip)]
    pub vs_currency: String,

    /// ids of coins, comma-separated
    #[setters(skip)]
    pub ids: Option<String>,

    /// ids of coins, comma-separated
    #[setters(skip)]
    pub category: Option<String>, //TODO make enum if i care

    #[setters(bool)]
    pub order: Option<Order>,

    // 1..250 max
    #[setters(bool)]
    pub per_page: Option<u32>,

    #[setters(u32)]
    pub page: Option<u32>,

    #[setters(bool)]
    pub sparkline: Option<bool>,

    #[setters(bool)]
    pub price_change_percentage: Option<PriceChangePercentage>,
}

#[derive(Clone)]
pub enum Order {
    GeckoDesc,
    GeckoAsc,
    MarketCapAsc,
    MarketCapDesc,
    VolumeAsc,
    VolumeDesc,
    IdAsc,
    IdDesc,
}
impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Order::GeckoDesc => "gecko_desc",
            Order::GeckoAsc => "gecko_asc",
            Order::MarketCapAsc => "market_cap_asc",
            Order::MarketCapDesc => "market_cap_desc",
            Order::VolumeAsc => "volume_asc",
            Order::VolumeDesc => "volume_desc",
            Order::IdAsc => "id_asc",
            Order::IdDesc => "id_desc",
        };
        write!(f, "{}", s)
    }
}
#[derive(Clone)]
pub enum PriceChangePercentage {
    OneHour,
    TentyFourHours,
    SevenDays,
    FourteenDays,
    ThirtyDays,
    TwoHundredDays,
    OneYear,
}
impl Display for PriceChangePercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PriceChangePercentage::OneHour => "1h",
            PriceChangePercentage::TentyFourHours => "24h",
            PriceChangePercentage::SevenDays => "7d",
            PriceChangePercentage::FourteenDays => "14d",
            PriceChangePercentage::ThirtyDays => "30d",
            PriceChangePercentage::TwoHundredDays => "200d",
            PriceChangePercentage::OneYear => "1y",
        };
        write!(f, "{}", s)
    }
}

impl MarketRequest {
    pub fn new(
        vs_currency: String,
        ids: Option<String>,
        category: Option<String>,
        order: Option<Order>,
        per_page: Option<u32>,
        page: Option<u32>,
        sparkline: Option<bool>,
        price_change_percentage: Option<PriceChangePercentage>,
    ) -> Self {
        MarketRequest {
            ids,
            vs_currency,
            category,
            order,
            per_page,
            page,
            sparkline,
            price_change_percentage,
        }
    }

    pub fn query(&self) -> String {
        fomat!(
            "vs_currency=" (self.vs_currency)
            if let Some(ids) = &self.ids {
                "&ids=" (ids)
            }
            if let Some(category) = &self.category {
                "&category=" (category)
            }
            if let Some(order) = &self.order {
                "&order=" (order)
            }
            if let Some(per_page) = self.per_page {
                "&per_page=" (per_page)
            }
            if let Some(page) = self.page {
                "&page=" (page)
            } else {
                "&page=1"
            }
            if let Some(sparkline) = self.sparkline {
                "&sparkline=" (sparkline)
            }
            if let Some(price_change_percentage) = &self.price_change_percentage {
                "&price_change_percentage=" (price_change_percentage)
            }
        )
    }
}
