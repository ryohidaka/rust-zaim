/// User information during authentication.
///
/// # Docs
/// @see https://dev.zaim.net/home/api#user_verify
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Me {
    pub id: i32,
    pub login: String,
    pub name: String,
    pub input_count: i32,
    pub day_count: i32,
    pub repeat_count: i32,
    pub day: i32,
    pub week: i32,
    pub month: i32,
    pub currency_code: String,
    pub profile_image_url: String,
    pub cover_image_url: String,
    pub profile_modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub me: Me,
    pub requested: i32,
}
