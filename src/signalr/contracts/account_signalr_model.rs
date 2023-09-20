use serde::{Deserialize, Serialize};
service_sdk::macros::use_signal_r_json_contract!();

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSignalRModel {
    pub id: String,
    pub balance: f64,
    pub bonus: f64,
    pub currency: String,
    pub is_live: bool,
    pub digits: i32,
    pub symbol: String,
    pub timestamp: u64,
    pub invest_amount: f64,
    pub achievement_status: String,
    pub free_to_withdrawal: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("accounts")]
pub struct AccountsSignalRModel {
    pub now: i64,
    pub data: Vec<AccountSignalRModel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("updateaccount")]
pub struct UpdateAccountSignalRModel {
    pub now: i64,
    pub data: AccountSignalRModel,
}
