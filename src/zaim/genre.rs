use reqwest::Method;

use crate::types::{default::genre::DefaultGenreResponse, home::genre::GenreResponse};

use super::request::{send_request, send_request_unauth};

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
pub async fn fetch_default_genres() -> DefaultGenreResponse {
    let endpoint = "genre";

    let res = send_request_unauth(endpoint, Method::GET)
        .await
        .json::<DefaultGenreResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}

/// Fetch Genres
///
/// Showing the list of your genres.
///
/// # Docs
/// @see https://dev.zaim.net/home/api#genre_home_get
///
/// # Example
/// Showing the list of your genres.
/// ```
/// use zaim::zaim::genre;
///
/// mod types;
///
/// #[tokio::main]
/// async fn main() {
///   // Fetch Genres
///   let genres = genre::fetch_genres().await.genres;
///   println!("{:?}", genres);
/// }
/// ```
pub async fn fetch_genres() -> GenreResponse {
    let endpoint = "genre";

    let res = send_request(endpoint, Method::GET)
        .await
        .json::<GenreResponse>()
        .await
        .expect("failed to convert struct from json");

    res
}
