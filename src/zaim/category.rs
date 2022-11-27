use reqwest::Method;

use crate::types::default::category::CategoryResponse;

use super::request::send_request;

/// Fetch Default Categories
///
/// Get default category list.
///
/// # Docs
/// @see https://dev.zaim.net/home/api#category_get
///
/// # Example
/// Get default category list.
/// ```
/// mod types;
/// use zaim::zaim::category;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Default Categories
///   let categories = category::fetch_default_categories().await.categories;
///   println!("{:?}", categories);
/// }
/// ```
pub async fn fetch_default_categories() -> CategoryResponse {
    let endpoint = "category";

    let res = send_request(endpoint, Method::GET)
        .await
        .json::<CategoryResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
