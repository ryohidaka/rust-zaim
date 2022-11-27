use reqwest::Method;

use crate::types::default::currency::CurrencyResponse;

use super::request::send_request;

/// Fetch Default Currencies
///
/// Showing the list of currencies.
///
/// # Docs
/// @see https://dev.zaim.net/home/api#currency_get
///
/// # Example
/// Showing the list of currencies.
/// ```
/// mod types;
/// use zaim::zaim::currency;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Default Currencies
///   let currencies = currency::fetch_default_currencies().await.currencies;
///   println!("{:?}", currencies);
/// }
/// ```
pub async fn fetch_default_currencies() -> CurrencyResponse {
    let endpoint = "currency";

    let res = send_request(endpoint, Method::GET)
        .await
        .json::<CurrencyResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
