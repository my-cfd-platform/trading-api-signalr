use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;

use crate::SignalRError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountData {
    pub account_id: String,
    pub trading_group_id: String,
    pub trading_profile_id: String,
}

#[derive(Debug, Clone)]
pub struct ClientData {
    pub trader_id: Option<String>,
    pub account_data: Option<AccountData>,
}

impl ClientData {
    pub fn new() -> Self {
        Self {
            trader_id: None,
            account_data: None,
        }
    }
}

#[derive(Debug)]
pub struct SignalRConnectionContext {
    pub client_data: RwLock<ClientData>,
}

impl SignalRConnectionContext {
    pub async fn set_active_account(&self, account_data: AccountData) {
        let mut write_access = self.client_data.write().await;
        write_access.account_data = Some(account_data);
    }

    pub async fn set_trader_id(&self, trader_id: &str) {
        let mut write_access = self.client_data.write().await;
        write_access.trader_id = Some(trader_id.to_string());
    }

    pub async fn get_trader_id(&self) -> Result<String, SignalRError> {
        let reed = self.client_data.read().await;

        let Some(trader_id) = &reed.trader_id else{
            return Err(SignalRError::TraderIdNotFound);
        };
        return Ok(trader_id.clone());
    }

    pub async fn get_account_data(&self) -> Result<AccountData, SignalRError> {
        let reed = self.client_data.read().await;

        let Some(account_info) = &reed.account_data else{
            return Err(SignalRError::TraderIdNotFound);
        };
        return Ok(account_info.clone());
    }
}

impl Default for SignalRConnectionContext {
    fn default() -> Self {
        Self {
            client_data: RwLock::new(ClientData::new()),
        }
    }
}
