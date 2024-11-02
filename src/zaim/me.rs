use super::client::Zaim;
use crate::models::me::MeResponse;

impl<'a> Zaim<'a> {
    /// Fetch Me
    ///
    /// Fetch user information during authentication.
    ///
    /// # Docs
    /// @see https://dev.zaim.net/home/api#user_verify
    ///
    /// # Example
    ///
    /// ```rust
    /// use dotenv::dotenv;
    /// use std::env;
    /// use zaim::zaim::client::Zaim;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok(); // Load environment variables
    ///
    ///     let zaim = Zaim::new(
    ///         env::var("ZAIM_CUSTOMER_ID").unwrap(),
    ///         env::var("ZAIM_CUSTOMER_SECRET").unwrap(),
    ///         env::var("ZAIM_TOKEN").unwrap(),
    ///         env::var("ZAIM_SECRET").unwrap(),
    ///         env::var("ZAIM_VERIFIER").unwrap(),
    ///     );
    ///
    ///     match zaim.fetch_me().await {
    ///         Ok(me_response) => {
    ///             let me = me_response.me;
    ///             println!("{:?}", me);
    ///         },
    ///         Err(e) => {
    ///             eprintln!("Error fetching user information: {}", e);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn fetch_me(&self) -> Result<MeResponse, Box<dyn std::error::Error>> {
        let endpoint = "home/user/verify";
        let params = None;

        let res = self.get(endpoint, params).await?;
        let me_response = res.json::<MeResponse>().await?;

        Ok(me_response)
    }
}
