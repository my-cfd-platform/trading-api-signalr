use std::sync::Arc;

use cfd_engine_sb_contracts::AccountBalanceUpdateSbModel;
use service_sdk::{
    async_trait,
    my_service_bus::abstractions::subscriber::{
        MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
    },
};

use crate::{AccountSignalRModel, AppContext, SignalRMessageWrapper, USER_ID_TAG};

pub struct AccountsUpdatesListener {
    pub app: Arc<AppContext>,
}

impl AccountsUpdatesListener {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<AccountBalanceUpdateSbModel> for AccountsUpdatesListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<AccountBalanceUpdateSbModel>,
    ) -> Result<(), MySbSubscriberHandleError> {
        while let Some(message) = messages_reader.get_next_message() {
            let operation = message.take_message();
            let account = operation.account_after_update.unwrap();
            let Some(connections) = self
                .app
                .connections
                .get_tagged_connections_with_value(USER_ID_TAG, &account.trader_id)
                .await
            else {
                continue;
            };

            for connection in connections {
                self.app
                    .signalr_message_sender
                    .send_message(
                        &connection,
                        crate::SignalROutcomeMessage::AccountUpdate(SignalRMessageWrapper::new(
                            AccountSignalRModel::from(account.clone()),
                        )),
                    )
                    .await;
            }
        }

        return Ok(());
    }
}
