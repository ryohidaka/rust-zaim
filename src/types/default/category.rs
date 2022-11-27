/// Default Category
///
/// # Docs
/// @see https://dev.zaim.net/home/api#category_get
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub mode: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultCategoryResponse {
    pub categories: Vec<Category>,
    pub requested: i32,
}
