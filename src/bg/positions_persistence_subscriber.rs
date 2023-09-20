use std::sync::Arc;

use cfd_engine_sb_contracts::PositionPersistenceEvent;
use service_sdk::{
    async_trait,
    my_service_bus::abstractions::subscriber::{
        MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
    },
    my_telemetry::MyTelemetryContext,
};

use crate::{
    trading_executor_grpc::TradingExecutorGetActivePositionsGrpcRequest,
    utils::init_signal_r_contract_now, ActivePositionSignalRModel, ActivePositionsSignalRModel,
    AppContext, UpdateActivePositionSignalRModel, USER_ID_TAG,
};

pub struct PositionsUpdateListener {
    pub app: Arc<AppContext>,
}

impl PositionsUpdateListener {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<PositionPersistenceEvent> for PositionsUpdateListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<PositionPersistenceEvent>,
    ) -> Result<(), MySbSubscriberHandleError> {
        while let Some(message) = messages_reader.get_next_message() {
            let event = message.take_message();

            if let Some(order_sb_model) = event.create_position {
                update_as_positions_list(
                    &self.app,
                    &order_sb_model.trader_id,
                    &order_sb_model.account_id,
                )
                .await;
            }

            if let Some(order_sb_model) = event.close_position {
                update_as_positions_list(
                    &self.app,
                    &order_sb_model.trader_id,
                    &order_sb_model.account_id,
                )
                .await;
            }

            if let Some(order_sb_model) = event.update_position {
                let Some(connections) = self
                    .app
                    .connections
                    .get_tagged_connections_with_value(USER_ID_TAG, &order_sb_model.trader_id)
                    .await
                else {
                    continue;
                };
                let position: ActivePositionSignalRModel = order_sb_model.into();

                for connection in &connections {
                    self.app
                        .signal_r_message_sender
                        .position_update_publisher
                        .send_to_connection(
                            &connection,
                            UpdateActivePositionSignalRModel {
                                now: init_signal_r_contract_now(),
                                data: position.clone(),
                            },
                        )
                        .await;
                }
            }
        }

        return Ok(());
    }
}

async fn update_as_positions_list(app: &Arc<AppContext>, trader_id: &str, account_id: &str) {
    let Some(connections) = app
        .connections
        .get_tagged_connections_with_value(USER_ID_TAG, trader_id)
        .await
    else {
        return;
    };

    let account_id = account_id.to_string();
    let positions = generate_positions_snapshot_message(app, trader_id, &account_id).await;

    for connection in &connections {
        app.signal_r_message_sender
            .active_position_publisher
            .send_to_connection(
                &connection,
                ActivePositionsSignalRModel {
                    now: init_signal_r_contract_now(),
                    data: positions.clone(),
                    account_id: account_id.clone(),
                },
            )
            .await;
    }
}

async fn generate_positions_snapshot_message(
    app: &Arc<AppContext>,
    trader_id: &str,
    account_id: &str,
) -> Vec<ActivePositionSignalRModel> {
    let account_active_positions = app
        .trading_executor
        .get_account_active_positions(
            TradingExecutorGetActivePositionsGrpcRequest {
                trader_id: trader_id.to_string(),
                account_id: account_id.to_string(),
            },
            &MyTelemetryContext::new(),
        )
        .await
        .unwrap();

    let account_active_positions = match account_active_positions {
        Some(result) => result,
        None => vec![],
    };

    account_active_positions
        .into_iter()
        .map(|x| x.into())
        .collect()
}
