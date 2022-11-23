use dotenv::dotenv;
use reqwest::{Client, Method, Response};
use reqwest_oauth1::{OAuthClientProvider, Secrets};
use std::env;

struct Env {
    base_url: String,
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    token_secret: String,
    oauth_verifier: String,
}

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

/// Get Environments
fn get_env() -> Env {
    dotenv().ok();

    let base_url = env::var("ZAIM_BASE_URL").expect("ZAIM_BASE_URL must be set.");
    let consumer_key = env::var("ZAIM_CUSTOMER_ID").expect("ZAIM_CUSTOMER_ID must be set.");
    let consumer_secret =
        env::var("ZAIM_CUSTOMER_SECRET").expect("ZAIM_CUSTOMER_SECRET must be set.");
    let access_token = env::var("ZAIM_TOKEN").expect("ZAIM_TOKEN must be set.");
    let token_secret = env::var("ZAIM_SECRET").expect("ZAIM_SECRET must be set.");
    let oauth_verifier = env::var("ZAIM_VERIFIER").expect("ZAIM_VERIFIER must be set.");

    Env {
        base_url,
        consumer_key,
        consumer_secret,
        access_token,
        token_secret,
        oauth_verifier,
    }
}
