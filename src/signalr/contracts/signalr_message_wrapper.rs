use my_signalr_middleware::{SignalrContractDeserializer, SignalrContractSerializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct SignalREmptyMessage();

impl SignalrContractDeserializer for SignalREmptyMessage {
    type Item = SignalREmptyMessage;

    fn deserialize(_: &[&[u8]]) -> Result<Self::Item, String> {
        return Ok(SignalREmptyMessage {});
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignalRMessageWrapperEmpty {
    pub now: u64,
}

impl SignalRMessageWrapperEmpty {
    pub fn new() -> Self {
        Self {
            now: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}

impl SignalrContractSerializer for SignalRMessageWrapperEmpty {
    fn serialize(self) -> Vec<Vec<u8>> {
        let json = serde_json::to_vec(&self);
        return vec![json.unwrap()];
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct SignalRMessageWrapperWithAccount<T>
where
    T: Serialize,
{
    pub now: u64,
    pub data: T,
    pub account_id: String,
}

impl<T> SignalRMessageWrapperWithAccount<T>
where
    T: Serialize,
{
    pub fn new(data: T, account_id: &str) -> Self {
        Self {
            now: chrono::Utc::now().timestamp_millis() as u64,
            data,
            account_id: account_id.to_string(),
        }
    }
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
