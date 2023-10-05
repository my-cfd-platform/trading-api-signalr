use std::sync::Arc;

use cfd_engine_sb_contracts::PendingPositionPersistenceEvent;
use service_sdk::{
    async_trait,
    my_service_bus::abstractions::subscriber::{
        MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
    },
    my_telemetry::MyTelemetryContext,
};

use crate::{
    trading_executor_grpc::TradingExecutorGetAccountPendingPositionGrpcRequest,
    utils::init_signal_r_contract_now, AppContext, PendingPositionSignalRModel,
    PendingPositionsSignalRModel, USER_ID_TAG,
};

pub struct PendingPositionsUpdateListener {
    pub app: Arc<AppContext>,
}

impl PendingPositionsUpdateListener {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<PendingPositionPersistenceEvent> for PendingPositionsUpdateListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<PendingPositionPersistenceEvent>,
    ) -> Result<(), MySbSubscriberHandleError> {
        while let Some(message) = messages_reader.get_next_message() {
            let my_telemetry = message.my_telemetry.engage_telemetry();
            let event = message.take_message();

            if let Some(order_sb_model) = event.create {
                update_as_positions_list(
                    &self.app,
                    &order_sb_model.trader_id,
                    &order_sb_model.account_id,
                    &my_telemetry,
                )
                .await;
                continue;
            }

            if let Some(order_sb_model) = event.execute {
                update_as_positions_list(
                    &self.app,
                    &order_sb_model.trader_id,
                    &order_sb_model.account_id,
                    &my_telemetry,
                )
                .await;
                continue;
            }

            if let Some(order_sb_model) = event.execute {
                update_as_positions_list(
                    &self.app,
                    &order_sb_model.trader_id,
                    &order_sb_model.account_id,
                    &my_telemetry,
                )
                .await;
                continue;
            }
        }

        return Ok(());
    }
}

async fn update_as_positions_list(
    app: &Arc<AppContext>,
    trader_id: &str,
    account_id: &str,
    my_telemetry_context: &MyTelemetryContext,
) {
    let Some(connections) = app
        .connections
        .get_tagged_connections_with_value(USER_ID_TAG, trader_id)
        .await
    else {
        return;
    };

    let account_id = account_id.to_string();
    let positions =
        generate_positions_snapshot_message(app, trader_id, &account_id, my_telemetry_context)
            .await;

    for connection in &connections {
        app.signal_r_message_sender
            .pending_position_publisher
            .send_to_connection(
                &connection,
                PendingPositionsSignalRModel {
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
    my_telemetry_context: &MyTelemetryContext,
) -> Vec<PendingPositionSignalRModel> {
    let account_active_positions = app
        .trading_executor
        .get_account_pending_positions(
            TradingExecutorGetAccountPendingPositionGrpcRequest {
                trader_id: trader_id.to_string(),
                account_id: account_id.to_string(),
            },
            my_telemetry_context,
        )
        .await
        .unwrap();

    let account_pending_positions = match account_active_positions {
        Some(result) => result,
        None => vec![],
    };

    account_pending_positions
        .into_iter()
        .map(|x| x.into())
        .collect()
}
