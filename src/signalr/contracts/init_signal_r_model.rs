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
    fn deserialize(src: &[&[u8]]) -> Result<Self::Item, String> {
        if src.len() != 1 {
            return Err(format!(
                "Invalid messages amount {} during deserialization for action: {}",
                src.len(),
                Self::ACTION_NAME
            ));
        }
        let payload = src.get(0).unwrap();
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

        Ok(result)
    }
}
