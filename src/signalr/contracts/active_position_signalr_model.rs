use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ActivePositionSignalRSideModel{
    Buy = 0,
    Sell = 1,
}

impl From<i32> for ActivePositionSignalRSideModel {
    fn from(value: i32) -> Self {
        match value{
            0 => ActivePositionSignalRSideModel::Buy,
            1 => ActivePositionSignalRSideModel::Sell,
            _ => panic!("Unknown value")
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
pub struct ActivePositionSignalRModel{
        pub id: String,
        pub investment_amount: f64,
        pub open_price: f64,
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