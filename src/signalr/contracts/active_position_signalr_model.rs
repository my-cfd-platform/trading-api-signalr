use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ActivePositionSignalRSideModel{
    Buy = 0,
    Sell = 1,
}

impl From<i32> for ActivePositionSignalRSideModel {
    fn from(value: i32) -> Self {
        match value{
            0 => ActivePositionSignalRSideModel::Buy,
            1 => ActivePositionSignalRSideModel::Buy,
            _ => panic!("Unknown value")
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePositionSignalRModel{
        pub id: String,
        pub invest_amount: f64,
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
        pub tp_type: Option<i32>,
        pub sl_type: Option<i32>,
        pub is_topping_up_active: bool,
        pub reserved_funds_for_topping_up: f64,
} 