use std::{collections::HashMap, sync::Arc};

use rust_extensions::MyTimerTick;

use crate::{AppContext, BidAskSignalRModel, SignalRMessageWrapper, SignalROutcomeMessage};

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
        println!("Send prices to clients job");
        let Some(connections) = self.app.connections.get_all().await else{
            return ;
        };

        let prices = self.app.bid_ask_aggregator.read().await;
        let prices = prices
            .get_current_profile()
            .values()
            .cloned()
            .collect::<Vec<HashMap<String, BidAskSignalRModel>>>();

        if prices.len() == 0 {
            return;
        }

        for item in prices.iter() {
            let instruments: Vec<BidAskSignalRModel> = item.values().cloned().collect();
            for connection in &connections {
                self.app
                    .signalr_message_sender
                    .send_message(
                        &connection,
                        SignalROutcomeMessage::BidAsk(SignalRMessageWrapper::new(
                            instruments.clone(),
                        )),
                    )
                    .await;
            }
        }

        todo!()
    }
}
