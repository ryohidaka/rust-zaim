use reqwest::{Client, Method, Response};
use reqwest_oauth1::{OAuthClientProvider, Secrets};
use std::collections::HashMap;

use crate::constants::BASE_URL;

use super::dotenv::get_env;

pub struct Zaim<'a> {
    pub http_client: Client,
    pub secrets: Secrets<'a>,
    pub oauth_verifier: String,
}

impl<'a> Zaim<'a> {
    pub fn new() -> Self {
        let env = get_env();
        let secrets = Secrets::new(env.consumer_key, env.consumer_secret)
            .token(env.access_token, env.token_secret);

        Self {
            http_client: Client::new(),
            secrets,
            oauth_verifier: env.oauth_verifier,
        }
    }

    /// Send a request to the Zaim API with the specified method and optional parameters
    pub async fn send_request(
        &self,
        endpoint: &str,
        method: Method,
        params: Option<HashMap<&str, String>>,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let query = self.build_query();

        let mut req = self
            .http_client
            .clone()
            .oauth1(self.secrets.clone())
            .request(method, &url)
            .query(&query);

        // Add query parameters if provided
        if let Some(params) = params {
            req = req.query(&params);
        }

        // Send the request and return the response
        req.send().await.map_err(Into::into)
    }

    /// Build the base query parameters for the request
    fn build_query(&self) -> Vec<(&str, String)> {
        vec![
            ("oauth_verifier", self.oauth_verifier.clone()),
            ("oauth_version", "1.0".to_string()),
        ]
    }

    /// Perform a GET request to the specified path with optional parameters
    pub async fn get(
        &self,
        endpoint: &str,
        params: Option<HashMap<&str, String>>,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        self.send_request(endpoint, Method::GET, params).await
    }
}
