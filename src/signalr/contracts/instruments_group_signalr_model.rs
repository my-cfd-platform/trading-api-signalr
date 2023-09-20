use serde::{Deserialize, Serialize};
service_sdk::macros::use_signal_r_json_contract!();

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentGroupSignalRModel {
    pub id: String,
    pub name: String,
    pub weight: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("instrumentgroups")]
pub struct InstrumentGroupsSignalRModel {
    pub now: i64,
    pub data: Vec<InstrumentGroupSignalRModel>,
    pub account_id: String,
}
