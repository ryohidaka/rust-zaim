use reqwest::Method;

use crate::types::default::currency::DefaultCurrencyResponse;

use super::request::send_request_unauth;

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
/// use zaim::zaim::currency;
///
/// mod types;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Default Currencies
///   let currencies = currency::fetch_default_currencies().await.currencies;
///   println!("{:?}", currencies);
/// }
/// ```
pub async fn fetch_default_currencies() -> DefaultCurrencyResponse {
    let endpoint = "currency";

    let res = send_request_unauth(endpoint, Method::GET)
        .await
        .json::<DefaultCurrencyResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
