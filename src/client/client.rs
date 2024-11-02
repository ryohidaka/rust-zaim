use reqwest::{Client, Method, Response};
use reqwest_oauth1::{OAuthClientProvider, Secrets};
use std::collections::HashMap;

use crate::constants::BASE_URL;

pub struct Zaim<'a> {
    pub http_client: Client,
    pub secrets: Secrets<'a>,
    pub oauth_verifier: String,
}

impl<'a> Zaim<'a> {
    /// Initialize a new Zaim client
    /// Required secrets should be provided as arguments
    ///
    /// # Arguments
    /// - `consumer_key`: Consumer key
    /// - `consumer_secret`: Consumer secret
    /// - `access_token`: Access token
    /// - `token_secret`: Token secret
    /// - `oauth_verifier`: OAuth verifier
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        token_secret: String,
        oauth_verifier: String,
    ) -> Self {
        let secrets = Secrets::new(consumer_key, consumer_secret).token(access_token, token_secret);

        Self {
            http_client: Client::new(),
            secrets,
            oauth_verifier,
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
