use rust_decimal::Decimal;
use std::collections::HashMap;

pub type SimplePrice = HashMap<String, Decimal>;
pub type SimplePrices = HashMap<String, SimplePrice>;

#[derive(Default, Setters, Clone)]
pub struct SimplePriceReq {
    /// ids of coins, comma-separated
    #[setters(skip)]
    pub ids: String,

    /// ids of currency pairs, comma-separated
    #[setters(skip)]
    pub vs_currencies: String,

    #[setters(bool)]
    pub include_market_cap: bool,

    #[setters(bool)]
    pub include_24hr_vol: bool,

    #[setters(bool)]
    pub include_24hr_change: bool,

    #[setters(bool)]
    pub include_last_updated_at: bool,
}

impl SimplePriceReq {
    pub fn new(ids: String, vs_currencies: String) -> Self {
        Self {
            ids,
            vs_currencies,
            ..Default::default()
        }
    }

    pub fn query(&self) -> String {
        fomat!(
            "ids=" (self.ids)
            "&vs_currencies=" (self.vs_currencies)
            if (self.include_market_cap) {
                "&include_market_cap=true"
            }
            if (self.include_24hr_vol) {
                "&include_24hr_vol=true"
            }
            if (self.include_24hr_change) {
                "&include_24hr_change=true"
            }
            if (self.include_last_updated_at) {
                "&include_last_updated_at=true"
            }
        )
    }
}
