use serde::{Deserialize, Serialize};
service_sdk::macros::use_signal_r_json_contract!();

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
//#[signal_r_json_contract("SetActiveAccount")]
pub struct SetActiveAccountModel {
    pub account_id: String,
}

impl my_http_server::signal_r::SignalRContractSerializer for SetActiveAccountModel {
    const ACTION_NAME: &'static str = "SetActiveAccount";
    type Item = SetActiveAccountModel;
    fn serialize(&self) -> Vec<Vec<u8>> {
        let json = serde_json::to_vec(&self);
        return vec![json.unwrap()];
    }
    fn deserialize(src: &[&[u8]]) -> Result<Self::Item, String> {
        println!("Debuging SetActiveAccountModel deserialize");
        println!("src.len(): {}", src.len());

        let mut no = 0;
        for itm in src {
            println!("itm[{}]: {:?}", no, std::str::from_utf8(itm));
            no += 1;
        }

        if src.len() != 1 {
            return Err(format!(
                "Invalid messages amount {} during deserialization for action: {}",
                src.len(),
                Self::ACTION_NAME
            ));
        }

        let payload = src.get(0).unwrap();
        let result = serde_json::from_slice(payload);
        if let Err(err) = &result {
            return Err(format!(
                "Invalid message during deserialization for action: {}. Error: {}",
                Self::ACTION_NAME,
                err
            ));
        }
        result.unwrap()
    }
}

impl SetActiveAccountModel {
    pub fn new(account_id: String) -> Self {
        Self { account_id }
    }
}
