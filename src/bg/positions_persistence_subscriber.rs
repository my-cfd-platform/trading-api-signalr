use std::sync::Arc;

use cfd_engine_sb_contracts::PositionPersistenceEvent;
use my_service_bus_abstractions::subscriber::{
    MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
};

use crate::{AppContext, SignalRMessageWrapperWithAccount, USER_ID_TAG, ActivePositionSignalRModel};

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
            let operation = message.take_message();

            let Some(active_position) = operation.create_position else{
                return Ok(());
            };

            let Some(connections) = self.app.connections
            .get_tagged_connections_with_value(USER_ID_TAG, &active_position.trader_id).await else{
                return Ok(());
            };

            for connection in connections {
                self.app
                    .signalr_message_sender
                    .send_message(
                        &connection,
                        crate::SignalROutcomeMessage::PositionUpdate(
                            SignalRMessageWrapperWithAccount::new(
                                ActivePositionSignalRModel::from(active_position.clone()),
                                &active_position.account_id,
                            ),
                        ),
                    )
                    .await;
            }
        }

        return Ok(());
    }
}