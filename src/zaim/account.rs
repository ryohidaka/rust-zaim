use reqwest::Method;

use crate::types::default::account::DefaultAccountResponse;

use super::request::send_request_unauth;

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
pub async fn fetch_default_accounts() -> DefaultAccountResponse {
    let endpoint = "account";

    let res = send_request_unauth(endpoint, Method::GET)
        .await
        .json::<DefaultAccountResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
