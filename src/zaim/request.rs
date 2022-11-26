use reqwest::{Client, Method, Response};
use reqwest_oauth1::{OAuthClientProvider, Secrets};

use crate::zaim::dotenv;

/// Send Request
pub async fn send_request(endpoint: &str, method: Method) -> Response {
    let env = dotenv::get_env();
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
