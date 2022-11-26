use reqwest::Method;

use crate::types::me::MeResponse;

use self::request::send_request;

pub mod dotenv;
pub mod request;

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
/// mod types;
/// mod zaim;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Me
///   let me = zaim::fetch_me().await.me;
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
