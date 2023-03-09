use std::{collections::HashMap, sync::Arc};

use my_nosql_contracts::SessionEntity;
use my_signalr_middleware::{
    MySignalrActionCallbacks, MySignalrConnection, SignalRPublshersBuilder, SignalrMessagePublisher,
};

use crate::{
    AccountSignalRModel, AppContext, BidAskSignalRModel, SignalREmptyMessage, SignalRError,
    SignalRIncomeMessage, SignalRInitAction, SignalRMessageWrapper, SignalRMessageWrapperEmpty,
    SignalROutcomeMessage, USER_ID,
};

pub struct SignalRPingMessageProcessor {
    app_context: Arc<AppContext>,
}

impl SignalRPingMessageProcessor {
    pub fn new(app_context: Arc<AppContext>) -> Self {
        Self { app_context }
    }

    pub async fn handle_message(
        &self,
        message: SignalRIncomeMessage,
        connection: &Arc<MySignalrConnection<()>>,
    ) {
        handle_message(&self.app_context, message, connection).await;
    }
}

#[async_trait::async_trait]
impl MySignalrActionCallbacks<SignalREmptyMessage> for SignalRPingMessageProcessor {
    type TCtx = ();

    async fn on(
        &self,
        connection: &Arc<MySignalrConnection<Self::TCtx>>,
        _: Option<HashMap<String, String>>,
        _: SignalREmptyMessage,
    ) {
        self.handle_message(SignalRIncomeMessage::Ping, connection)
            .await;
    }
}

pub struct SignalRInitMessageProcessor {
    app_context: Arc<AppContext>,
}

#[async_trait::async_trait]
impl MySignalrActionCallbacks<SignalRInitAction> for SignalRInitMessageProcessor {
    type TCtx = ();

    async fn on(
        &self,
        connection: &Arc<MySignalrConnection<Self::TCtx>>,
        _: Option<HashMap<String, String>>,
        data: SignalRInitAction,
    ) {
        self.handle_message(SignalRIncomeMessage::Init(data), connection)
            .await;
    }
}

impl SignalRInitMessageProcessor {
    pub fn new(app_context: Arc<AppContext>) -> Self {
        Self { app_context }
    }

    pub async fn handle_message(
        &self,
        message: SignalRIncomeMessage,
        connection: &Arc<MySignalrConnection<()>>,
    ) {
        handle_message(&self.app_context, message, connection).await;
    }
}

pub struct SignalRMessageSender {
    accounts_publisher:
        SignalrMessagePublisher<SignalRMessageWrapper<Vec<AccountSignalRModel>>, ()>,
    bidask_publisher: SignalrMessagePublisher<SignalRMessageWrapper<Vec<BidAskSignalRModel>>, ()>,
    error_publisher: SignalrMessagePublisher<SignalRError, ()>,
    pong_publisher: SignalrMessagePublisher<SignalRMessageWrapperEmpty, ()>,
}

impl SignalRMessageSender {
    pub fn new(builder: &Arc<SignalRPublshersBuilder<()>>) -> Self {
        Self {
            accounts_publisher: builder.get_publisher("accounts".to_string()),
            error_publisher: builder.get_publisher("servererror".to_string()),
            bidask_publisher: builder.get_publisher("bidask".to_string()),
            pong_publisher: builder.get_publisher("pong".to_string()),
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
            SignalROutcomeMessage::Pong(date) => self.send_pong(connection, date).await,
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

    async fn send_pong(
        &self,
        connection: &Arc<MySignalrConnection<()>>,
        date: SignalRMessageWrapperEmpty,
    ) {
        self.pong_publisher
            .send_to_connection(connection, date)
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

async fn handle_message(
    app: &Arc<AppContext>,
    message: SignalRIncomeMessage,
    connection: &Arc<MySignalrConnection<()>>,
) {
    match message {
        SignalRIncomeMessage::Init(token) => {
            let session = app
                .sessions_ns_reader
                .get_entity(&SessionEntity::get_pk(), &token.token)
                .await;

            let Some(session) =  session else{
                app.signalr_message_sender.send_message(
                    connection,
                    SignalROutcomeMessage::Error(SignalRError::new("Session not found".to_string())),
                ).await;

                return ;
            };

            app.connections
                .add_tag_to_connection(&connection, USER_ID, &session.trader_id)
                .await;

            let accounts = app
                .accounts_manager
                .get_client_accounts(&session.trader_id)
                .await;

            app.signalr_message_sender
                .send_message(
                    connection,
                    SignalROutcomeMessage::Accounts(SignalRMessageWrapper::new(accounts)),
                )
                .await;
        }
        SignalRIncomeMessage::SetActiveAccount(_) => todo!(),
        SignalRIncomeMessage::Ping => {
            app.signalr_message_sender
                .send_message(
                    connection,
                    SignalROutcomeMessage::Pong(SignalRMessageWrapperEmpty::new()),
                )
                .await
        }
    }
}
