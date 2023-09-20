use serde::{Deserialize, Serialize};
service_sdk::macros::use_signal_r_json_contract!();

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentSignalRModel {
    pub id: String,
    pub name: String,
    pub digits: i32,
    pub base: String,
    pub quote: String,
    pub day_off: Vec<InstumentSignalRDayOffModel>,
    pub min_operation_volume: f64,
    pub max_operation_volume: f64,
    pub amount_step_size: f64,
    pub max_position_volume: f64,
    pub stop_out_percent: f64,
    pub multiplier: Vec<i32>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub group_id: Option<String>,
    pub sub_group_id: Option<String>,
    pub weight: Option<i32>,
    pub markup_bid: Option<f64>,
    pub markup_ask: Option<f64>,
    pub tick_size: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstumentSignalRDayOffModel {
    pub dow_from: i32,
    pub time_from: String,
    pub dow_to: i32,
    pub time_to: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("instruments")]
pub struct InstrumentsSignalRModel {
    pub now: i64,
    pub data: Vec<InstrumentSignalRModel>,
    pub account_id: String,
}
