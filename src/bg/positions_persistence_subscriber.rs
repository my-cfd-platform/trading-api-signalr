use std::sync::Arc;

use cfd_engine_sb_contracts::PositionPersistenceEvent;
use my_service_bus_abstractions::subscriber::{
    MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
};

use crate::{
    ActivePositionSignalRModel, AppContext, SbPositionPersistenceUpdateType,
    SignalRMessageWrapperWithAccount, SignalROutcomeMessage, USER_ID_TAG,
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
            let operation: SbPositionPersistenceUpdateType =
                SbPositionPersistenceUpdateType::from(message.take_message());

            let trader_id = operation.extract_trader_id();

            let Some(connections) = self.app.connections
                .get_tagged_connections_with_value(USER_ID_TAG, &trader_id).await else{
                    continue;
                };

            let mut messages_to_send = vec![];

            match operation {
                SbPositionPersistenceUpdateType::Create(order) => {
                    messages_to_send.push(SignalROutcomeMessage::PositionUpdate(
                        SignalRMessageWrapperWithAccount::new(
                            ActivePositionSignalRModel::from(order.clone()),
                            &order.account_id,
                        ),
                    ));

                    messages_to_send.push(
                        generate_positions_snapshot_message(
                            &self.app,
                            &order.trader_id,
                            &order.account_id,
                        )
                        .await,
                    );
                }
                SbPositionPersistenceUpdateType::Update(order) => {
                    messages_to_send.push(SignalROutcomeMessage::PositionUpdate(
                        SignalRMessageWrapperWithAccount::new(
                            ActivePositionSignalRModel::from(order.clone()),
                            &order.account_id,
                        ),
                    ));

                    messages_to_send.push(
                        generate_positions_snapshot_message(
                            &self.app,
                            &order.trader_id,
                            &order.account_id,
                        )
                        .await,
                    );
                }
                SbPositionPersistenceUpdateType::Close(order) => {
                    messages_to_send.push(
                        generate_positions_snapshot_message(
                            &self.app,
                            &order.trader_id,
                            &order.account_id,
                        )
                        .await,
                    );
                }
            };

            for connection in connections {
                for message in &messages_to_send {
                    self.app
                        .signalr_message_sender
                        .send_message(&connection, message.clone())
                        .await;
                }
            }
        }

        return Ok(());
    }
}

async fn generate_positions_snapshot_message(
    app: &Arc<AppContext>,
    trader_id: &str,
    account_id: &str,
) -> SignalROutcomeMessage {
    let account_active_positions = app
        .trading_executor
        .get_active_positions(trader_id, account_id)
        .await;

    return SignalROutcomeMessage::ActivePositions(SignalRMessageWrapperWithAccount::new(
        account_active_positions
            .iter()
            .map(|x| x.to_owned().into())
            .collect(),
        &account_id,
    ));
}
