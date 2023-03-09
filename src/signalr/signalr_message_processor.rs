use std::{collections::HashMap, sync::Arc};

use my_nosql_contracts::{
    SessionEntity, TradingGroupNoSqlEntity, TradingInstrumentNoSqlEntity, TradingProfileNoSqlEntity,
};
use my_signalr_middleware::{
    MySignalrActionCallbacks, MySignalrConnection, SignalRPublshersBuilder,
    SignalrContractDeserializer, SignalrMessagePublisher,
};
use serde::{Deserialize, Serialize};

use crate::{
    AccountSignalRModel, AppContext, BidAskSignalRModel, InstumentSignalRModel,
    SetActiveAccountCommand, SignalRConnectionContext, SignalREmptyMessage, SignalRError,
    SignalRIncomeMessage, SignalRInitAction, SignalRMessageWrapper, SignalRMessageWrapperEmpty,
    SignalRMessageWrapperWithAccount, SignalROutcomeMessage, USER_ID,
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
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
    ) {
        handle_message(&self.app_context, message, connection).await;
    }
}

#[async_trait::async_trait]
impl MySignalrActionCallbacks<SignalREmptyMessage> for SignalRPingMessageProcessor {
    type TCtx = SignalRConnectionContext;

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
    type TCtx = SignalRConnectionContext;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalRSetActiveAccountMessage {
    pub account_id: String,
}

impl SignalrContractDeserializer for SignalRSetActiveAccountMessage {
    type Item = SignalRSetActiveAccountMessage;

    fn deserialize(data: &[&[u8]]) -> Result<Self::Item, String> {
        return Ok(serde_json::from_slice(data[0]).unwrap());
    }
}

pub struct SignalRSetActiveAccountMessageProcessor {
    app_context: Arc<AppContext>,
}

impl SignalRSetActiveAccountMessageProcessor {
    pub fn new(app_context: Arc<AppContext>) -> Self {
        Self { app_context }
    }

    pub async fn handle_message(
        &self,
        message: SignalRIncomeMessage,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
    ) {
        handle_message(&self.app_context, message, connection).await;
    }
}

#[async_trait::async_trait]
impl MySignalrActionCallbacks<SignalRSetActiveAccountMessage>
    for SignalRSetActiveAccountMessageProcessor
{
    type TCtx = SignalRConnectionContext;

    async fn on(
        &self,
        connection: &Arc<MySignalrConnection<Self::TCtx>>,
        _: Option<HashMap<String, String>>,
        data: SignalRSetActiveAccountMessage,
    ) {

        connection.ctx.set_active_account(&data.account_id).await;
        self.handle_message(
            SignalRIncomeMessage::SetActiveAccount(SetActiveAccountCommand::new(data.account_id)),
            connection,
        )
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
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
    ) {
        handle_message(&self.app_context, message, connection).await;
    }
}

pub struct SignalRMessageSender {
    accounts_publisher: SignalrMessagePublisher<
        SignalRMessageWrapper<Vec<AccountSignalRModel>>,
        SignalRConnectionContext,
    >,
    bidask_publisher: SignalrMessagePublisher<
        SignalRMessageWrapper<Vec<BidAskSignalRModel>>,
        SignalRConnectionContext,
    >,
    error_publisher: SignalrMessagePublisher<SignalRError, SignalRConnectionContext>,
    pong_publisher: SignalrMessagePublisher<SignalRMessageWrapperEmpty, SignalRConnectionContext>,
    instruments_publisher: SignalrMessagePublisher<
        SignalRMessageWrapperWithAccount<Vec<InstumentSignalRModel>>,
        SignalRConnectionContext,
    >,
}

impl SignalRMessageSender {
    pub fn new(builder: &Arc<SignalRPublshersBuilder<SignalRConnectionContext>>) -> Self {
        Self {
            accounts_publisher: builder.get_publisher("accounts".to_string()),
            error_publisher: builder.get_publisher("servererror".to_string()),
            bidask_publisher: builder.get_publisher("bidask".to_string()),
            pong_publisher: builder.get_publisher("pong".to_string()),
            instruments_publisher: builder.get_publisher("instruments".to_string()),
        }
    }

    pub async fn send_message(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        message: SignalROutcomeMessage,
    ) {
        match message {
            SignalROutcomeMessage::Instruments(instruments) => {
                self.send_instruments(connection, instruments).await
            }
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
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        accounts: SignalRMessageWrapper<Vec<AccountSignalRModel>>,
    ) {
        self.accounts_publisher
            .send_to_connection(connection, accounts)
            .await;
    }

    async fn send_error(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        error: SignalRError,
    ) {
        self.error_publisher
            .send_to_connection(connection, error)
            .await;
    }
    async fn send_instruments(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        message: SignalRMessageWrapperWithAccount<Vec<InstumentSignalRModel>>,
    ) {
        self.instruments_publisher
            .send_to_connection(connection, message)
            .await;
    }

    async fn send_pong(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        date: SignalRMessageWrapperEmpty,
    ) {
        self.pong_publisher
            .send_to_connection(connection, date)
            .await;
    }

    async fn send_bid_ask(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
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
    connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
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
                .add_tag_to_connection(connection, USER_ID, &session.trader_id)
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
        SignalRIncomeMessage::SetActiveAccount(set_account_message) => {
            let trading_account = app
                .accounts_manager
                .get_client_account(&connection.ctx.trader_id, &set_account_message.account_id)
                .await;

            let Some(trading_account) = trading_account else{
                app.signalr_message_sender.send_message(connection, SignalROutcomeMessage::Error(SignalRError::new("Account not found".to_string()))).await;
                return;
            };

            let Some(trading_group) = app
                .trading_groups_ns_reader
                .get_entity(
                    TradingGroupNoSqlEntity::generate_partition_key(),
                    &trading_account.trading_group,
                )
                .await else{
                    app.signalr_message_sender.send_message(connection, SignalROutcomeMessage::Error(SignalRError::new("Group not found".to_string()))).await;
                    return;
                };

            let Some(trading_profile) = app.trading_profile_ns_reader.get_entity(TradingProfileNoSqlEntity::generate_partition_key(), &trading_group.trading_profile_id).await else{
                app.signalr_message_sender.send_message(connection, SignalROutcomeMessage::Error(SignalRError::new("Trading profile not found".to_string()))).await;
                return;
            };

            let instruments = trading_profile.instruments.iter().map(|tp_instument| async {
                let Some(instument_model) = app.instruments_ns_reader.get_entity(TradingInstrumentNoSqlEntity::generate_partition_key(), &tp_instument.id).await else{
                    panic!("instrument existed in tp but not in instruments")
                };

                return InstumentSignalRModel{
                    id: tp_instument.id.clone(),
                    name: instument_model.name.clone(),
                    digits: instument_model.digits,
                    base: instument_model.base.clone(),
                    quote: instument_model.quote.clone(),
                    day_off: instument_model.days_off.iter().map(|day| day.to_owned().into()).collect(),
                    min_operation_volume: tp_instument.min_operation_volume,
                    max_operation_volume: tp_instument.max_operation_volume,
                    amount_step_size: 1.0,
                    max_position_volume: tp_instument.max_position_volume,
                    stop_out_percent: trading_profile.stop_out_percent,
                    multiplier: vec![5],
                    bid: None,
                    ask: None,
                    group_id: None,
                    sub_group_id: None,
                    weight: instument_model.weight,
                    markup_bid: None,
                    markup_ask: None,
                    tick_size: Some(instument_model.tick_size),
                };
            });

            let mut instruments_to_send = vec![];

            for task in instruments {
                let result: InstumentSignalRModel = task.await;
                instruments_to_send.push(result);
            }

            app.signalr_message_sender
                .send_message(
                    connection,
                    SignalROutcomeMessage::Instruments(
                        crate::SignalRMessageWrapperWithAccount::new(
                            instruments_to_send,
                            &set_account_message.account_id,
                        ),
                    ),
                )
                .await;
        }
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
