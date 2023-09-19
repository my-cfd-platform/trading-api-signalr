use serde::{Deserialize, Serialize};
use service_sdk::my_http_server::signalr::SignalrContractSerializer;

pub enum SignalRError {
    SessionNotFound,
    AccountsNotFound,
    InstrumentsNotFound,
    InstrumentsGroupsNotFound,
    PricesSnapshotNotFound,
    TradingGroupNotFound,
    TradingProfileNotFound,
    InvalidClientData,
    TraderIdNotFound,
    TradingDisabled,
    PriceChangeNotFound,
    NetworkError(String),
}

impl SignalRError {
    pub fn get_message(&self) -> SignalRErrorMessage {
        match self {
            SignalRError::SessionNotFound => {
                SignalRErrorMessage::new("Session not found".to_string())
            }
            SignalRError::AccountsNotFound => {
                SignalRErrorMessage::new("Account not found".to_string())
            }
            SignalRError::NetworkError(src) => {
                SignalRErrorMessage::new(format!("Network error: {}", src))
            }
            SignalRError::InstrumentsNotFound => {
                SignalRErrorMessage::new("Instruments not found".to_string())
            }
            SignalRError::InstrumentsGroupsNotFound => {
                SignalRErrorMessage::new("Instruments groups not found".to_string())
            }
            SignalRError::PricesSnapshotNotFound => {
                SignalRErrorMessage::new("Prices snapshot not found".to_string())
            }
            SignalRError::TradingGroupNotFound => {
                SignalRErrorMessage::new("Trading group not found".to_string())
            }
            SignalRError::TradingProfileNotFound => {
                SignalRErrorMessage::new("Trading profile not found".to_string())
            }
            SignalRError::InvalidClientData => {
                SignalRErrorMessage::new("Invalid client data".to_string())
            }
            SignalRError::TraderIdNotFound => {
                SignalRErrorMessage::new("Trader id not found".to_string())
            }
            SignalRError::TradingDisabled => {
                SignalRErrorMessage::new("Trading disabled".to_string())
            }
            SignalRError::PriceChangeNotFound => {
                SignalRErrorMessage::new("Price change not found".to_string())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignalRErrorMessage {
    pub error: String,
}

impl SignalRErrorMessage {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

impl SignalrContractSerializer for SignalRErrorMessage {
    fn serialize(self) -> Vec<Vec<u8>> {
        let json = serde_json::to_vec(&self);
        return vec![json.unwrap()];
    }
}
