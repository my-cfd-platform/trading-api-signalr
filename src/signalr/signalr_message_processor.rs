use std::{collections::HashMap, sync::Arc};

use my_nosql_contracts::SessionEntity;
use my_signalr_middleware::{
    MySignalrActionCallbacks, MySignalrConnection, SignalRPublshersBuilder, SignalrMessagePublisher,
};

use crate::{
    AccountSignalRModel, AppContext, BidAskSignalRModel, SignalRError, SignalRIncomeMessage,
    SignalRInitAction, SignalRMessageWrapper, SignalROutcomeMessage, USER_ID,
};

pub struct SignalRMessageProcessor {
    app_context: Arc<AppContext>,
}

#[async_trait::async_trait]
impl MySignalrActionCallbacks<SignalRInitAction> for SignalRMessageProcessor {
    type TCtx = ();

    async fn on(
        &self,
        connection: &Arc<MySignalrConnection<Self::TCtx>>,
        _: Option<HashMap<String, String>>,
        data: SignalRInitAction,
    ) {
        self.handle_message(SignalRIncomeMessage::Init(data), connection).await;
    }
}

impl SignalRMessageProcessor {
    pub fn new(app_context: Arc<AppContext>) -> Self {
        Self { app_context }
    }

    pub async fn handle_message(
        &self,
        message: SignalRIncomeMessage,
        connection: &Arc<MySignalrConnection<()>>,
    ) {
        match message {
            SignalRIncomeMessage::Init(token) => {
                let session = self
                    .app_context
                    .sessions_ns_reader
                    .get_entity(&SessionEntity::get_pk(), &token.token)
                    .await;

                let Some(session) =  session else{
                    self.app_context.signalr_message_sender.send_message(
                        connection,
                        SignalROutcomeMessage::Error(SignalRError::new("Session not found".to_string())),
                    ).await;

                    return ;
                };

                self.app_context
                    .connections
                    .add_tag_to_connection(&connection, USER_ID, &session.trader_id)
                    .await;

                let accounts = self
                    .app_context
                    .accounts_manager
                    .get_client_accounts(&session.trader_id)
                    .await;

                self.app_context
                    .signalr_message_sender
                    .send_message(
                        connection,
                        SignalROutcomeMessage::Accounts(SignalRMessageWrapper::new(accounts)),
                    )
                    .await;
            }
            SignalRIncomeMessage::SetActiveAccount(_) => todo!(),
        }
    }
}

pub struct SignalRMessageSender {
    accounts_publisher:
        SignalrMessagePublisher<SignalRMessageWrapper<Vec<AccountSignalRModel>>, ()>,
    bidask_publisher: SignalrMessagePublisher<SignalRMessageWrapper<Vec<BidAskSignalRModel>>, ()>,
    error_publisher: SignalrMessagePublisher<SignalRError, ()>,
}

impl SignalRMessageSender {
    pub fn new(builder: &Arc<SignalRPublshersBuilder<()>>) -> Self {
        Self {
            accounts_publisher: builder.get_publisher("accounts".to_string()),
            error_publisher: builder.get_publisher("servererror".to_string()),
            bidask_publisher: builder.get_publisher("bidask".to_string()),
        }
    }

    pub async fn send_message(
        &self,
        connection: &Arc<MySignalrConnection<()>>,
        message: SignalROutcomeMessage,
    ) {
        match message {
            SignalROutcomeMessage::Instuments(_) => todo!(),
            SignalROutcomeMessage::PriceChange(_) => todo!(),
            SignalROutcomeMessage::PositionsActive(_) => todo!(),
            SignalROutcomeMessage::PendingOrders(_) => todo!(),
            SignalROutcomeMessage::Accounts(accounts) => {
                self.send_accounts(connection, accounts).await
            }
            SignalROutcomeMessage::BidAsk(bid_ask) => self.send_bid_ask(connection, bid_ask).await,
            SignalROutcomeMessage::Error(error) => self.send_error(connection, error).await,
        };
    }

    async fn send_accounts(
        &self,
        connection: &Arc<MySignalrConnection<()>>,
        accounts: SignalRMessageWrapper<Vec<AccountSignalRModel>>,
    ) {
        self.accounts_publisher
            .send_to_connection(connection, accounts)
            .await;
    }

    async fn send_error(&self, connection: &Arc<MySignalrConnection<()>>, error: SignalRError) {
        self.error_publisher
            .send_to_connection(connection, error)
            .await;
    }
    async fn send_bid_ask(
        &self,
        connection: &Arc<MySignalrConnection<()>>,
        bidask: SignalRMessageWrapper<Vec<BidAskSignalRModel>>,
    ) {
        self.bidask_publisher
            .send_to_connection(connection, bidask)
            .await;
    }
}
