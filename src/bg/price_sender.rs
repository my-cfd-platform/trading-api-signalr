use std::sync::Arc;

use crate::{
    utils::init_signal_r_contract_now, AppContext, BidAskSignalRModel, BidAsksSignalRModel,
};
use fx_utils::MarkupUtils;
use service_sdk::async_trait;
use service_sdk::rust_extensions::MyTimerTick;

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
            let mut prices = self.app.bid_ask_aggregator.lock().await;
            prices.get_current_profile()
        };

        let Some(prices) = prices else {
            return;
        };

        if prices.len() == 0 {
            return;
        }

        for connection in &connections {
            let trading_group_id = connection.ctx.get_trader_group().await;

            if trading_group_id.is_none() {
                continue;
            }

            let trading_group_id = trading_group_id.unwrap();

            let mut data = Vec::with_capacity(prices.len());

            for bid_ask in &prices {
                let markup_applier = self
                    .app
                    .get_markup_applier(&bid_ask.id, &trading_group_id)
                    .await;

                match markup_applier {
                    Some(markup_applier) => {
                        data.push(BidAskSignalRModel {
                            id: bid_ask.id.clone(),
                            bid: bid_ask.bid.apply_markup(markup_applier.bid),
                            ask: bid_ask.ask.apply_markup(markup_applier.ask),
                            dt: bid_ask.dt,
                            dir: bid_ask.dir,
                        });
                    }
                    None => {
                        data.push(bid_ask.clone());
                    }
                }
            }

            let contract = BidAsksSignalRModel {
                now: init_signal_r_contract_now(),
                data,
            };

            self.app
                .signal_r_message_sender
                .bid_ask_publisher
                .send_to_connection(connection, &contract)
                .await;
        }
    }
}
