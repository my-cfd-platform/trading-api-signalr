use std::sync::Arc;

use cfd_engine_sb_contracts::BidAskSbModel;
use service_sdk::{
    async_trait,
    my_service_bus::abstractions::subscriber::{
        MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
    },
};

use crate::AppContext;

pub struct PricesListener {
    pub app: Arc<AppContext>,
}

impl PricesListener {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<BidAskSbModel> for PricesListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<BidAskSbModel>,
    ) -> Result<(), MySbSubscriberHandleError> {
        let mut write = self.app.bid_ask_aggregator.lock().await;
        while let Some(message) = messages_reader.get_next_message() {
            let message = message.take_message();
            write.update(&message);
        }

        return Ok(());
    }
}
