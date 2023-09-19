use std::sync::Arc;

use service_sdk::my_http_server::signalr::{
    MySignalrConnection, SignalRPublshersBuilder, SignalrMessagePublisher,
};

use crate::{
    AccountSignalRModel, ActivePositionSignalRModel, BidAskSignalRModel,
    InstrumentGroupSignalRModel, InstrumentSignalRModel, PriceChangeSignalRModel,
    SignalRConnectionContext, SignalRErrorMessage, SignalRMessageWrapper,
    SignalRMessageWrapperEmpty, SignalRMessageWrapperWithAccount, SignalROutcomeMessage,
};

pub struct SignalRMessageSender {
    accounts_publisher: SignalrMessagePublisher<
        SignalRMessageWrapper<Vec<AccountSignalRModel>>,
        SignalRConnectionContext,
    >,
    bidask_publisher: SignalrMessagePublisher<
        SignalRMessageWrapper<Vec<BidAskSignalRModel>>,
        SignalRConnectionContext,
    >,
    error_publisher: SignalrMessagePublisher<SignalRErrorMessage, SignalRConnectionContext>,
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
        connection: &MySignalrConnection<SignalRConnectionContext>,
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
        connection: &MySignalrConnection<SignalRConnectionContext>,
        accounts: SignalRMessageWrapper<Vec<AccountSignalRModel>>,
    ) {
        self.accounts_publisher
            .send_to_connection(connection, accounts)
            .await;
    }
    async fn send_accounts_update(
        &self,
        connection: &MySignalrConnection<SignalRConnectionContext>,
        accounts: SignalRMessageWrapper<AccountSignalRModel>,
    ) {
        self.account_update_publisher
            .send_to_connection(connection, accounts)
            .await;
    }
    async fn send_position_update(
        &self,
        connection: &MySignalrConnection<SignalRConnectionContext>,
        position: SignalRMessageWrapperWithAccount<ActivePositionSignalRModel>,
    ) {
        self.position_update_publisher
            .send_to_connection(connection, position)
            .await;
    }

    async fn send_active_position(
        &self,
        connection: &MySignalrConnection<SignalRConnectionContext>,
        position: SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>,
    ) {
        self.position_publisher
            .send_to_connection(connection, position)
            .await;
    }
    async fn send_price_change(
        &self,
        connection: &MySignalrConnection<SignalRConnectionContext>,
        change: SignalRMessageWrapper<Vec<PriceChangeSignalRModel>>,
    ) {
        self.price_change_publisher
            .send_to_connection(connection, change)
            .await;
    }

    async fn send_error(
        &self,
        connection: &MySignalrConnection<SignalRConnectionContext>,
        error: SignalRErrorMessage,
    ) {
        self.error_publisher
            .send_to_connection(connection, error)
            .await;
    }
    async fn send_instruments(
        &self,
        connection: &MySignalrConnection<SignalRConnectionContext>,
        message: SignalRMessageWrapperWithAccount<Vec<InstrumentSignalRModel>>,
    ) {
        self.instruments_publisher
            .send_to_connection(connection, message)
            .await;
    }

    async fn send_pong(
        &self,
        connection: &MySignalrConnection<SignalRConnectionContext>,
        date: SignalRMessageWrapperEmpty,
    ) {
        self.pong_publisher
            .send_to_connection(connection, date)
            .await;
    }
    async fn send_instrument_groups(
        &self,
        connection: &MySignalrConnection<SignalRConnectionContext>,
        date: SignalRMessageWrapperWithAccount<Vec<InstrumentGroupSignalRModel>>,
    ) {
        self.instruments_groups_publisher
            .send_to_connection(connection, date)
            .await;
    }

    async fn send_bid_ask(
        &self,
        connection: &MySignalrConnection<SignalRConnectionContext>,
        bidask: SignalRMessageWrapper<Vec<BidAskSignalRModel>>,
    ) {
        self.bidask_publisher
            .send_to_connection(connection, bidask)
            .await;
    }
}
