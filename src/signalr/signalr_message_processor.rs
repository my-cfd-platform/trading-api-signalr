use std::{collections::HashMap, sync::Arc};

use my_nosql_contracts::{
    BidAskSnapshotNoSqlEntity, PriceChangeSnapshotNoSqlEntity, TradingGroupNoSqlEntity,
    TradingInstrumentNoSqlEntity, TradingProfileNoSqlEntity,
};
use my_signalr_middleware::{
    MySignalrActionCallbacks, MySignalrConnection, SignalRPublshersBuilder,
    SignalrContractDeserializer, SignalrMessagePublisher,
};
use my_telemetry::MyTelemetryContext;
use rest_api_wl_shared::middlewares::SessionEntity;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use serde::{Deserialize, Serialize};

use crate::{
    accounts_manager_grpc::*, trading_executor_grpc::*, AccountSignalRModel,
    ActivePositionSignalRModel, AppContext, BidAskSignalRModel, InstrumentGroupSignalRModel,
    InstrumentSignalRModel, PriceChangeSignalRModel, SetActiveAccountCommand,
    SignalRConnectionContext, SignalREmptyMessage, SignalRError, SignalRIncomeMessage,
    SignalRInitAction, SignalRMessageWrapper, SignalRMessageWrapperEmpty,
    SignalRMessageWrapperWithAccount, SignalROutcomeMessage, USER_ID_TAG,
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
        ctx: &MyTelemetryContext,
    ) {
        handle_message(&self.app_context, message, connection, ctx).await;
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
        ctx: &MyTelemetryContext,
    ) {
        self.handle_message(SignalRIncomeMessage::Ping, connection, ctx)
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
        ctx: &MyTelemetryContext,
    ) {
        self.handle_message(SignalRIncomeMessage::Init(data), connection, ctx)
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
        ctx: &MyTelemetryContext,
    ) {
        handle_message(&self.app_context, message, connection, ctx).await;
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
        ctx: &MyTelemetryContext,
    ) {
        connection.ctx.set_active_account(&data.account_id).await;
        self.handle_message(
            SignalRIncomeMessage::SetActiveAccount(SetActiveAccountCommand::new(data.account_id)),
            connection,
            ctx,
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
        ctx: &MyTelemetryContext,
    ) {
        handle_message(&self.app_context, message, connection, ctx).await;
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
        SignalRMessageWrapperWithAccount<Vec<InstrumentSignalRModel>>,
        SignalRConnectionContext,
    >,
    account_update_publisher: SignalrMessagePublisher<
        SignalRMessageWrapper<AccountSignalRModel>,
        SignalRConnectionContext,
    >,
    price_change_publisher: SignalrMessagePublisher<
        SignalRMessageWrapper<Vec<PriceChangeSignalRModel>>,
        SignalRConnectionContext,
    >,

    instruments_groups_publisher: SignalrMessagePublisher<
        SignalRMessageWrapperWithAccount<Vec<InstrumentGroupSignalRModel>>,
        SignalRConnectionContext,
    >,
    position_update_publisher: SignalrMessagePublisher<
        SignalRMessageWrapperWithAccount<ActivePositionSignalRModel>,
        SignalRConnectionContext,
    >,

    position_publisher: SignalrMessagePublisher<
        SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>,
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
            account_update_publisher: builder.get_publisher("updateaccount".to_string()),
            price_change_publisher: builder.get_publisher("pricechange".to_string()),
            instruments_groups_publisher: builder.get_publisher("instrumentgroups".to_string()),
            position_update_publisher: builder.get_publisher("updateactiveposition".to_string()),
            position_publisher: builder.get_publisher("positionsactive".to_string()),
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
            SignalROutcomeMessage::PriceChange(change) => {
                self.send_price_change(connection, change).await
            }
            SignalROutcomeMessage::PositionsActive(_) => todo!(),
            SignalROutcomeMessage::PendingOrders(_) => todo!(),
            SignalROutcomeMessage::Accounts(accounts) => {
                self.send_accounts(connection, accounts).await
            }
            SignalROutcomeMessage::AccountUpdate(account) => {
                self.send_accounts_update(connection, account).await
            }
            SignalROutcomeMessage::BidAsk(bid_ask) => self.send_bid_ask(connection, bid_ask).await,
            SignalROutcomeMessage::Error(error) => self.send_error(connection, error).await,
            SignalROutcomeMessage::Pong(date) => self.send_pong(connection, date).await,
            SignalROutcomeMessage::InstrumentsGroups(groups) => {
                self.send_instrument_groups(connection, groups).await
            }
            SignalROutcomeMessage::PositionUpdate(update_message) => {
                self.send_position_update(connection, update_message).await
            }
            SignalROutcomeMessage::ActivePositions(active_positions) => {
                self.send_active_position(connection, active_positions)
                    .await
            }
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
    async fn send_accounts_update(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        accounts: SignalRMessageWrapper<AccountSignalRModel>,
    ) {
        self.account_update_publisher
            .send_to_connection(connection, accounts)
            .await;
    }
    async fn send_position_update(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        position: SignalRMessageWrapperWithAccount<ActivePositionSignalRModel>,
    ) {
        self.position_update_publisher
            .send_to_connection(connection, position)
            .await;
    }

    async fn send_active_position(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        position: SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>,
    ) {
        self.position_publisher
            .send_to_connection(connection, position)
            .await;
    }
    async fn send_price_change(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        change: SignalRMessageWrapper<Vec<PriceChangeSignalRModel>>,
    ) {
        self.price_change_publisher
            .send_to_connection(connection, change)
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
        message: SignalRMessageWrapperWithAccount<Vec<InstrumentSignalRModel>>,
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
    async fn send_instrument_groups(
        &self,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
        date: SignalRMessageWrapperWithAccount<Vec<InstrumentGroupSignalRModel>>,
    ) {
        self.instruments_groups_publisher
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
    ctx: &MyTelemetryContext,
) {
    match message {
        SignalRIncomeMessage::Init(token) => {
            let session = app
                .sessions_ns_reader
                .get_entity(&SessionEntity::get_pk(), &token.token)
                .await;

            let Some(session) = session else {
                app.signalr_message_sender
                    .send_message(
                        connection,
                        SignalROutcomeMessage::Error(SignalRError::new(
                            "Session not found".to_string(),
                        )),
                    )
                    .await;

                return;
            };
            connection.ctx.set_trader_id(&session.trader_id).await;
            println!(
                "Init after auth. Ctx: {:#?}, Message: {:#?}",
                connection.ctx, token
            );
            app.connections
                .add_tag_to_connection(connection, USER_ID_TAG, &session.trader_id)
                .await;

            let accounts = app
                .accounts_manager
                .get_client_accounts(
                    AccountManagerGetClientAccountsGrpcRequest {
                        trader_id: session.trader_id.to_string(),
                    },
                    ctx,
                )
                .await
                .unwrap();

            println!("Accounts: {:#?}", accounts);

            let accounts = match accounts {
                Some(accounts) => accounts.iter().map(|acc| acc.to_owned().into()).collect(),
                None => vec![],
            };

            app.signalr_message_sender
                .send_message(
                    connection,
                    SignalROutcomeMessage::Accounts(SignalRMessageWrapper::new(accounts)),
                )
                .await;
        }
        SignalRIncomeMessage::SetActiveAccount(set_account_message) => {
            connection
                .ctx
                .set_active_account(&set_account_message.account_id)
                .await;
            let client_data = connection.ctx.get_client_data().await;
            println!(
                "Set active account. Ctx: {:#?}, Message: {:#?}",
                connection.ctx, set_account_message
            );
            let response = app
                .accounts_manager
                .get_client_account(
                    AccountManagerGetClientAccountGrpcRequest {
                        trader_id: client_data.trader_id.unwrap(),
                        account_id: client_data.active_account_id.unwrap(),
                    },
                    ctx,
                )
                .await
                .unwrap();

            let Some(trading_account) = response.account else {
                app.signalr_message_sender
                    .send_message(
                        connection,
                        SignalROutcomeMessage::Error(SignalRError::new(
                            "Account not found".to_string(),
                        )),
                    )
                    .await;
                return;
            };

            let Some(raw_instruments) = app.instruments_ns_reader.get_table_snapshot_as_vec().await
            else {
                app.signalr_message_sender
                    .send_message(
                        connection,
                        SignalROutcomeMessage::Error(SignalRError::new(
                            "Instruments not found".to_string(),
                        )),
                    )
                    .await;
                return;
            };

            let mut instruments: HashMap<String, Arc<TradingInstrumentNoSqlEntity>> =
                HashMap::new();

            for instrument in raw_instruments {
                if !instrument.trading_disabled {
                    instruments.insert(instrument.get_id().to_string(), instrument);
                }
            }

            let Some(instruments_groups) = app
                .instruments_groups_ns_reader
                .get_table_snapshot_as_vec()
                .await
            else {
                app.signalr_message_sender
                    .send_message(
                        connection,
                        SignalROutcomeMessage::Error(SignalRError::new(
                            "Instruments groups not found".to_string(),
                        )),
                    )
                    .await;
                return;
            };

            let instruments_groups_to_send: Vec<InstrumentGroupSignalRModel> = instruments_groups
                .iter()
                .filter_map(|x| {
                    if x.hidden {
                        return None;
                    }

                    return Some(InstrumentGroupSignalRModel {
                        id: x.id.clone(),
                        name: x.name.clone(),
                        weight: x.weight,
                    });
                })
                .collect();

            let Some(trading_group) = app
                .trading_groups_ns_reader
                .get_entity(
                    TradingGroupNoSqlEntity::generate_partition_key(),
                    &trading_account.trading_group,
                )
                .await
            else {
                app.signalr_message_sender
                    .send_message(
                        connection,
                        SignalROutcomeMessage::Error(SignalRError::new(
                            "Trading group not found".to_string(),
                        )),
                    )
                    .await;
                return;
            };

            let Some(trading_profile) = app
                .trading_profile_ns_reader
                .get_entity(
                    TradingProfileNoSqlEntity::generate_partition_key(),
                    &trading_group.trading_profile_id,
                )
                .await
            else {
                app.signalr_message_sender
                    .send_message(
                        connection,
                        SignalROutcomeMessage::Error(SignalRError::new(
                            "Trading profile not found".to_string(),
                        )),
                    )
                    .await;
                return;
            };

            let prices = app
                .bid_ask_snapshot_reader
                .get_by_partition_key(BidAskSnapshotNoSqlEntity::generate_partition_key())
                .await;

            let instruments_to_send: Vec<InstrumentSignalRModel> = trading_profile
                .instruments
                .iter()
                .map(|tp_instrument| {
                    let Some(instrument_model) = instruments.get(&tp_instrument.id) else {
                        return None;
                    };

                    if instrument_model.trading_disabled {
                        return None;
                    }

                    let mut instrument = InstrumentSignalRModel {
                        id: tp_instrument.id.clone(),
                        name: instrument_model.name.clone(),
                        digits: instrument_model.digits,
                        base: instrument_model.base.clone(),
                        quote: instrument_model.quote.clone(),
                        day_off: instrument_model
                            .days_off
                            .iter()
                            .map(|day| day.to_owned().into())
                            .collect(),
                        min_operation_volume: tp_instrument.min_operation_volume,
                        max_operation_volume: tp_instrument.max_operation_volume,
                        amount_step_size: 1.0,
                        max_position_volume: tp_instrument.max_position_volume,
                        stop_out_percent: trading_profile.stop_out_percent,
                        multiplier: tp_instrument.leverages.clone(),
                        bid: None,
                        ask: None,
                        group_id: instrument_model.group_id.clone(),
                        sub_group_id: None,
                        weight: instrument_model.weight,
                        markup_bid: None,
                        markup_ask: None,
                        tick_size: Some(instrument_model.tick_size),
                    };

                    if let Some(prices) = &prices {
                        if let Some(price) = prices.get(&instrument.id) {
                            instrument.bid = Some(price.bid);
                            instrument.ask = Some(price.ask);
                        }
                    }

                    return Some(instrument);
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect();

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

            app.signalr_message_sender
                .send_message(
                    connection,
                    SignalROutcomeMessage::InstrumentsGroups(
                        SignalRMessageWrapperWithAccount::new(
                            instruments_groups_to_send,
                            &set_account_message.account_id,
                        ),
                    ),
                )
                .await;

            let price_change = app
                .price_change_ns_reader
                .get_by_partition_key(PriceChangeSnapshotNoSqlEntity::get_daily_pk())
                .await;

            let active_positions = app
                .trading_executor
                .get_account_active_positions(
                    TradingExecutorGetActivePositionsGrpcRequest {
                        trader_id: trading_account.trader_id,
                        account_id: trading_account.id,
                    },
                    ctx,
                )
                .await
                .unwrap();

            let active_positions = if let Some(active_positions) = active_positions {
                active_positions
            } else {
                vec![]
            };

            app.signalr_message_sender
                .send_message(
                    connection,
                    SignalROutcomeMessage::ActivePositions(SignalRMessageWrapperWithAccount::new(
                        active_positions
                            .iter()
                            .map(|x| x.to_owned().into())
                            .collect(),
                        &set_account_message.account_id,
                    )),
                )
                .await;

            if let Some(price_changes) = price_change {
                let mut to_send = vec![];

                for (_, price_change) in price_changes {
                    if let Some(instrument) = instruments.get(&price_change.row_key) {
                        let change = (price_change.current_price - price_change.previous_price)
                            / price_change.previous_price
                            * 100.0;

                        let Some(change) = Decimal::from_f64(change) else {
                            continue;
                        };

                        let change = change.round_dp(instrument.tick_size as u32);

                        to_send.push(PriceChangeSignalRModel {
                            id: price_change.row_key.clone(),
                            chng: change.to_f64().unwrap(),
                        });
                    }
                }

                app.signalr_message_sender
                    .send_message(
                        connection,
                        SignalROutcomeMessage::PriceChange(SignalRMessageWrapper::new(to_send)),
                    )
                    .await;
            }
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
