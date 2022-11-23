use dotenv::dotenv;
use std::env;

struct Env {
    base_url: String,
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    token_secret: String,
    oauth_verifier: String,
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
