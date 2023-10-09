use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
service_sdk::macros::use_signal_r_json_contract!();

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ActivePositionSignalRSideModel {
    Buy = 0,
    Sell = 1,
}

impl From<i32> for ActivePositionSignalRSideModel {
    fn from(value: i32) -> Self {
        match value {
            0 => ActivePositionSignalRSideModel::Buy,
            1 => ActivePositionSignalRSideModel::Sell,
            _ => panic!("Unknown value"),
        }
    }
}

#[derive(Clone, Copy, Serialize_repr, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum SlTpType {
    Currency,
    Price,
    Percent,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePositionSignalRModel {
    pub id: String,
    pub investment_amount: f64,
    pub base: String,
    pub quote: String,
    pub collateral: String,
    pub open_price: f64,
    pub base_collateral_open_price: f64,
    pub open_date: u64,
    pub instrument: String,
    pub multiplier: f64,
    pub operation: ActivePositionSignalRSideModel,
    pub swap: f64,
    pub commission: f64,
    pub time_stamp: u64,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
    pub tp_type: Option<SlTpType>,
    pub sl_type: Option<SlTpType>,
    pub is_topping_up_active: bool,
    pub reserved_funds_for_topping_up: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("positionsactive")]
pub struct ActivePositionsSignalRModel {
    pub now: i64,
    pub data: Vec<ActivePositionSignalRModel>,
    pub account_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("updateactiveposition")]
pub struct UpdateActivePositionSignalRModel {
    pub now: i64,
    pub data: ActivePositionSignalRModel,
}
