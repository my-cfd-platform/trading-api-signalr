use my_signalr_middleware::SignalrContractDeserializer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SignalRInitAction {
    pub token: String,
}
impl SignalrContractDeserializer for SignalRInitAction {
    type Item = SignalRInitAction;
    fn deserialize(data: &[&[u8]]) -> Result<Self::Item, String> {
        if data.len() < 1 {
            return Err("Init signal-r contract must have at least 1 parameter".to_string());
        }

        let item = data[0];

        let token = &item[1..item.len() - 1];

        Ok(Self {
            token: String::from_utf8(token.to_vec()).unwrap(),
        })
    }
}
