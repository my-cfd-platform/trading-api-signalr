use my_signalr_middleware::SignalrContractSerializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignalRError {
    pub error: String,
}

impl SignalRError {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

impl SignalrContractSerializer for SignalRError {
    fn serialize(self) -> Vec<Vec<u8>> {
        let json = serde_json::to_vec(&self);
        return vec![json.unwrap()];
    }    
}