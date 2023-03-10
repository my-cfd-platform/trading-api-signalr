use std::sync::Arc;

use cfd_engine_sb_contracts::BidAskSbModel;
use my_service_bus_abstractions::subscriber::{SubscriberCallback, MessagesReader, MySbSubscriberHandleError};

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
        while let Some(message) = messages_reader.get_next_message() {
            let operation = message.take_message();
            let mut write = self.app.bid_ask_aggregator.write().await;
            write.update(&operation);
        }

        return Ok(());
    }
}
