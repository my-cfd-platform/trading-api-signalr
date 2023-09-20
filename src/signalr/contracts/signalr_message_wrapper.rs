service_sdk::macros::use_signal_r_json_contract!();
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};
use service_sdk::my_http_server::signal_r::{
    SignalRContractDeserializer, SignalRContractSerializer,
};

#[derive(Serialize, Debug, Clone)]
pub struct SignalREmptyMessage();

impl SignalRContractDeserializer for SignalREmptyMessage {
    type Item = SignalREmptyMessage;

    fn deserialize(_: &[&[u8]]) -> Result<Self::Item, String> {
        return Ok(SignalREmptyMessage {});
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignalRMessageWrapperEmpty {
    pub now: i64,
}

impl SignalRMessageWrapperEmpty {
    pub fn new() -> Self {
        Self {
            now: DateTimeAsMicroseconds::now().unix_microseconds / 1000,
        }
    }
}

impl SignalRContractSerializer for SignalRMessageWrapperEmpty {
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
    pub data: Option<T>,
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

/*
impl<T> SignalRContractSerializer for SignalRMessageWrapper<T>
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

impl<T> SignalRContractSerializer for SignalRMessageWrapperWithAccount<T>
where
    T: Serialize,
{
    fn serialize(self) -> Vec<Vec<u8>> {
        let json = serde_json::to_vec(&self);
        return vec![json.unwrap()];
    }
}
 */
