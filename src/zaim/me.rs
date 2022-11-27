use reqwest::Method;

use crate::types::home::me::MeResponse;

use super::request::send_request;

/// Fetch Me
///
/// Fetch user information during authentication.
///
/// # Docs
/// @see https://dev.zaim.net/home/api#user_verify
///
/// # Example
/// Fetch user information during authentication.
/// ```
/// use zaim::zaim::me;
///
/// mod types;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Me
///   let me = me::fetch_me().await.me;
///   println!("{:?}", me);
/// }
/// ```
pub async fn fetch_me() -> MeResponse {
    let endpoint = "user/verify";

    let res = send_request(endpoint, Method::GET)
        .await
        .json::<MeResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
