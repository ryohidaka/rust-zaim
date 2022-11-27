use reqwest::Method;

use crate::types::default::genre::GenreResponse;

use super::request::send_request;

/// Fetch Default Genres
///
/// Get default genre list.
///
/// # Docs
/// @see https://dev.zaim.net/home/api#genre_get
///
/// # Example
/// Get default genre list.
/// ```
/// use zaim::zaim::genre;
///
/// mod types;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Default Genres
///   let genres = genre::fetch_default_genres().await.genres;
///   println!("{:?}", genres);
/// }
/// ```
pub async fn fetch_default_genres() -> GenreResponse {
    let endpoint = "genre";

    let res = send_request(endpoint, Method::GET)
        .await
        .json::<GenreResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
