use reqwest::{Client, Method, Response};
use reqwest_oauth1::{OAuthClientProvider, Secrets};

use crate::types::me::MeResponse;

/// Send Request
async fn send_request(endpoint: &str, method: Method) -> Response {
    let env = get_env();
    let secrets = Secrets::new(env.consumer_key, env.consumer_secret)
        .token(env.access_token, env.token_secret);

    let query = vec![
        ("oauth_verifier", env.oauth_verifier),
        ("oauth_version", "1.0".to_string()),
    ];

    let client = Client::new();

    let url = format!("{}{}", env.base_url, endpoint);

    let res = client
        .oauth1(secrets)
        .request(method, url)
        .query(&query)
        .send()
        .await
        .expect("failed to get response");

    res
}

pub mod dotenv;

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
