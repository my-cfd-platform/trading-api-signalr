use serde::{Deserialize, Serialize};

service_sdk::macros::use_signal_r_json_contract!();

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceChangeSignalRModel {
    pub id: String,
    pub chng: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("pricechange")]
pub struct PriceChangesSignalRModel {
    pub now: i64,
    pub data: Vec<PriceChangeSignalRModel>,
}
