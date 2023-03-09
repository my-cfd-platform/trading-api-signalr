use std::sync::RwLock;

#[derive(Debug)]
pub struct SignalRConnectionContext{
    pub trader_id: String,
    pub active_account_id: RwLock<Option<String>>,
}

impl SignalRConnectionContext {
    pub fn new(trader_id: String) -> Self {
        Self {
            trader_id,
            active_account_id: RwLock::new(None),
        }
    }

    pub async fn set_active_account(&self, active_account_id: &str) {
        let mut write_access = self.active_account_id.write().unwrap();
        *write_access = Some(active_account_id.to_string());
    }
}

impl Default for SignalRConnectionContext {
    fn default() -> Self {
        Self { trader_id: Default::default(), active_account_id: Default::default() }
    }
}