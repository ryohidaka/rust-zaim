/// Default Currency
///
/// # Docs
/// @see https://dev.zaim.net/home/api#currency_get
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub currency_code: String,
    pub unit: String,
    pub name: String,
    pub point: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyResponse {
    pub currencies: Vec<Currency>,
    pub requested: i32,
}
