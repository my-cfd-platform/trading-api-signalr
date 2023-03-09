use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ClientData {
    pub trader_id: Option<String>,
    pub active_account_id: Option<String>,
}

impl ClientData {
    pub fn new() -> Self {
        Self {
            trader_id: None,
            active_account_id: None,
        }
    }
}

#[derive(Debug)]
pub struct SignalRConnectionContext {
    pub client_data: RwLock<ClientData>,
}

impl SignalRConnectionContext {
    pub async fn set_active_account(&self, active_account_id: &str) {
        let mut write_access = self.client_data.write().await;
        write_access.active_account_id = Some(active_account_id.to_string());
    }

    pub async fn set_trader_id(&self, trader_id: &str) {
        let mut write_access = self.client_data.write().await;
        write_access.trader_id = Some(trader_id.to_string());
    }

    pub async fn get_client_data(&self) -> ClientData{
        let reed = self.client_data.read().await;
        return reed.clone();
    }
}

impl Default for SignalRConnectionContext {
    fn default() -> Self {
        Self {
            client_data: RwLock::new(ClientData::new()),
        }
    }
}
