use serde::{Deserialize, Serialize};
service_sdk::macros::use_signal_r_json_contract!();

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingSignalRModel {}

impl my_http_server::signal_r::SignalRContractSerializer for PingSignalRModel {
    const ACTION_NAME: &'static str = "ping";
    type Item = PingSignalRModel;

    fn serialize(&self) -> Vec<Vec<u8>> {
        return vec![];
    }
    fn deserialize(_src: &[&[u8]]) -> Result<Self::Item, String> {
        Ok(PingSignalRModel {})
    }
}
