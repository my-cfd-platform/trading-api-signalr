use my_signalr_middleware::SignalrContractSerializer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SignalRMessageWrapper<T>
where
    T: Serialize,
{
    pub now: u64,
    pub data: T,
}

impl<T> SignalRMessageWrapper<T>
where
    T: Serialize,
{
    pub fn new(data: T) -> Self {
        Self {
            now: chrono::Utc::now().timestamp_millis() as u64,
            data,
        }
    }
}

impl<T> SignalrContractSerializer for SignalRMessageWrapper<T>
where
    T: Serialize,
{
    fn serialize(self) -> Vec<Vec<u8>> {
        let json = serde_json::to_vec(&self);
        return vec![json.unwrap()];
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SignalRMessageWrapperWithAccount<T>
where
    T: Serialize,
{
    pub now: u64,
    pub data: T,
    pub account_id: String,
}

impl<T> SignalrContractSerializer for SignalRMessageWrapperWithAccount<T>
where
    T: Serialize,
{
    fn serialize(self) -> Vec<Vec<u8>> {
        let json = serde_json::to_vec(&self);
        return vec![json.unwrap()];
    }
}
