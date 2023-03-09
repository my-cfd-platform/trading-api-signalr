use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BidAskSignalRModel{
    pub id: String,
    pub bid: BidAskDataSignalRModel,
    pub ask: BidAskDataSignalRModel,
    pub dt: u64,
    pub dir: i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BidAskDataSignalRModel{
    pub h: f64,
    pub l: f64,
    pub o: f64,
    pub c: f64,
}