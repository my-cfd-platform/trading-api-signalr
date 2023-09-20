use std::sync::Arc;

use rust_extensions::MyTimerTick;
use service_sdk::async_trait;

use crate::{utils::init_signal_r_contract_now, AppContext, BidAsksSignalRModel};

pub struct PriceSendTimer {
    app: Arc<AppContext>,
}

impl PriceSendTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for PriceSendTimer {
    async fn tick(&self) {
        let Some(connections) = self.app.connections.get_all().await else {
            return;
        };

        let prices = {
            let prices = self.app.bid_ask_aggregator.read().await;
            prices.get_current_profile().cloned()
        };

        let Some(prices) = prices else {
            return;
        };

        if prices.len() == 0 {
            return;
        }

        let mut instruments = Vec::with_capacity(prices.len());

        for (_, bid_ask) in prices {
            instruments.push(bid_ask);
        }

        for connection in &connections {
            self.app
                .signal_r_message_sender
                .bidask_publisher
                .send_to_connection(
                    connection,
                    BidAsksSignalRModel {
                        now: init_signal_r_contract_now(),
                        data: instruments.clone(),
                    },
                )
                .await;
        }
    }
}
