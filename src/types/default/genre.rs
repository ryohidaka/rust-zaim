/// Default Genre
///
/// # Docs
/// @see https://dev.zaim.net/home/api#genre_get
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreResponse {
    pub genres: Vec<Genre>,
    pub requested: i32,
}
