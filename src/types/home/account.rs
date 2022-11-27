/// Account
///
/// # Docs
/// @see https://dev.zaim.net/home/api#account_home_get
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    // pub mode: String,
    pub sort: i32,
    pub parent_account_id: i32,
    pub active: i32,
    pub modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub accounts: Vec<Account>,
    pub requested: i32,
}
