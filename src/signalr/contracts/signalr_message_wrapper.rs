use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SignalRMessageWrapper<T> {
    pub now: u64,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SignalRMessageWrapperWithAccount<T> {
    pub now: u64,
    pub data: T,
    pub account_id: String,
}
