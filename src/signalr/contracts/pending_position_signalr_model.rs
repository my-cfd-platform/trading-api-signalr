use serde::{Deserialize, Serialize};
use service_sdk::my_http_server::signal_r::macros::signal_r_json_contract;

use crate::{ActivePositionSignalRSideModel, SlTpType};
use service_sdk::my_http_server;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingPositionSignalRModel {
    pub id: String,
    pub investment_amount: f64,
    pub instrument: String,
    pub multiplier: f64,
    pub operation: ActivePositionSignalRSideModel,
    pub time_stamp: u64,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
    pub tp_type: Option<SlTpType>,
    pub sl_type: Option<SlTpType>,
    pub desire_price: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("pendingpositions")]
pub struct PendingPositionsSignalRModel {
    pub now: i64,
    pub data: Vec<PendingPositionSignalRModel>,
    pub account_id: String,
}