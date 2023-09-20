use serde::{Deserialize, Serialize};
service_sdk::macros::use_signal_r_json_contract!();

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("SetActiveAccount")]
pub struct SetActiveAccountModel {
    pub account_id: String,
}

impl SetActiveAccountModel {
    pub fn new(account_id: String) -> Self {
        Self { account_id }
    }
}
