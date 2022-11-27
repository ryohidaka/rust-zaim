use reqwest::Method;

use crate::types::default::account::AccountResponse;

use super::request::send_request;

/// Fetch Default Accounts
///
/// Get default account list.
///
/// # Docs
/// @see https://dev.zaim.net/home/api#account_get
///
/// # Example
/// Get default account list.
/// ```
/// mod types;
/// use zaim::zaim::account;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Default Account
///   let accounts = account::fetch_default_accounts().await.accounts;
///   println!("{:?}", accounts);
/// }
/// ```
pub async fn fetch_default_accounts() -> AccountResponse {
    let endpoint = "account";

    let res = send_request(endpoint, Method::GET)
        .await
        .json::<AccountResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
