/// Genre
///
/// # Docs
/// @see https://dev.zaim.net/home/api#genre_home_get
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: i32,
    pub name: String,
    pub sort: i32,
    pub active: i32,
    pub category_id: i32,
    pub parent_genre_id: i32,
    pub modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreResponse {
    pub genres: Vec<Genre>,
    pub requested: i32,
}
