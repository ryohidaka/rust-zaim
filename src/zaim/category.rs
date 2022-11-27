use reqwest::Method;

use crate::types::default::category::DefaultCategoryResponse;

use super::request::send_request_unauth;

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
/// use zaim::zaim::category;
///
/// mod types;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Default Categories
///   let categories = category::fetch_default_categories().await.categories;
///   println!("{:?}", categories);
/// }
/// ```
pub async fn fetch_default_categories() -> DefaultCategoryResponse {
    let endpoint = "category";

    let res = send_request_unauth(endpoint, Method::GET)
        .await
        .json::<DefaultCategoryResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
