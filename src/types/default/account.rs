/// Default Account
///
/// # Docs
/// @see https://dev.zaim.net/home/api#account_get
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub accounts: Vec<Account>,
    pub requested: i32,
}
