use crate::types::me::MeResponse;
use reqwest::Method;

use super::request::send_request;

/// Fetch Me
///
/// Fetch user information during authentication.
///
/// # Docs
/// @see https://dev.zaim.net/home/api#user_verify
///
/// # Example
/// ```rust
/// use zaim::zaim::me;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Me
///   let me = me::fetch_me().await.me;
///   println!("{:?}", me);
/// }
/// ```
pub async fn fetch_me() -> MeResponse {
    let endpoint = "home/user/verify";

    let res = send_request(endpoint, Method::GET)
        .await
        .json::<MeResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
