use dotenv::dotenv;
use std::env;

use crate::constants::BASE_URL;

pub struct Env {
    pub base_url: String,
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub token_secret: String,
    pub oauth_verifier: String,
}

/// Get Environments
pub fn get_env() -> Env {
    dotenv().ok();

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BASE_URL.to_string());
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
