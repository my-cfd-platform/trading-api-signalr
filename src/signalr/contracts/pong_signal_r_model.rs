use serde::{Deserialize, Serialize};
service_sdk::macros::use_signal_r_json_contract!();

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("pong")]
pub struct PongSignalRModel {
    pub now: i64,
}
