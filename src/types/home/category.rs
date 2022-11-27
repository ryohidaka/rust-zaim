/// Category
///
/// # Docs
/// @see https://dev.zaim.net/home/api#category_home_get
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub mode: String,
    pub sort: i32,
    pub parent_category_id: i32,
    pub active: i32,
    pub modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryResponse {
    pub categories: Vec<Category>,
    pub requested: i32,
}
