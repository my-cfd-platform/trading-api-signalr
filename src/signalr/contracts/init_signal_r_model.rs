service_sdk::macros::use_signal_r_json_contract!();
pub struct InitSignalRModel {
    pub session_token: String,
}

impl my_http_server::signal_r::SignalRContractSerializer for InitSignalRModel {
    const ACTION_NAME: &'static str = "init";
    type Item = InitSignalRModel;

    fn serialize(&self) -> Vec<Vec<u8>> {
        let json = self.session_token.as_bytes();
        return vec![json.to_vec()];
    }
    fn deserialize<'s>(src: impl Iterator<Item = &'s [u8]>) -> Result<Self::Item, String> {
        for payload in src {
            if payload.len() < 5 {
                return Err(format!(
                    "Invalid payload:{:?}",
                    std::str::from_utf8(payload)
                ));
            }

            let payload = &payload[1..&payload.len() - 1];

            let result = payload.to_vec();

            let result = String::from_utf8(result);
            if let Err(err) = &result {
                return Err(format!(
                    "Invalid message during deserialization for action: {}. Error: {}",
                    Self::ACTION_NAME,
                    err
                ));
            }
            let result = Self {
                session_token: result.unwrap(),
            };

            return Ok(result);
        }

        return Err(format!(
            "Can not be 0 parameters during deserialization for action: {}",
            Self::ACTION_NAME
        ));
    }
}
