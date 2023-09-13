use std::sync::Arc;

use my_no_sql_tcp_reader::{MyNoSqlDataReader, MyNoSqlTcpConnection};
use my_nosql_contracts::*;
use my_service_bus_tcp_client::MyServiceBusClient;
use my_signalr_middleware::{SignalRPublshersBuilder, SignalrConnectionsList};
use rest_api_wl_shared::middlewares::SessionEntity;
use rust_extensions::AppStates;
use tokio::sync::RwLock;

use crate::{
    settings::SettingsReader, AccountsManagerGrpcClient, BidAskAggregator,
    SignalRConnectionContext, SignalRMessageSender, TradingExecutorGrpcClient,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub instruments_ns_reader: Arc<MyNoSqlDataReader<TradingInstrumentNoSqlEntity>>,
    pub bid_ask_snapshot_reader: Arc<MyNoSqlDataReader<BidAskSnapshotNoSqlEntity>>,
    pub sessions_ns_reader: Arc<MyNoSqlDataReader<SessionEntity>>,
    pub trading_groups_ns_reader: Arc<MyNoSqlDataReader<TradingGroupNoSqlEntity>>,
    pub price_change_ns_reader: Arc<MyNoSqlDataReader<PriceChangeSnapshotNoSqlEntity>>,
    pub trading_profile_ns_reader: Arc<MyNoSqlDataReader<TradingProfileNoSqlEntity>>,
    pub instruments_groups_ns_reader: Arc<MyNoSqlDataReader<TradingInstrumentGroupNoSqlEntity>>,
    pub connections: Arc<SignalrConnectionsList<SignalRConnectionContext>>,
    pub accounts_manager: AccountsManagerGrpcClient,
    pub signalr_message_sender: Arc<SignalRMessageSender>,
    pub my_no_sql_connection: MyNoSqlTcpConnection,
    pub app_states: Arc<AppStates>,
    pub bid_ask_aggregator: Arc<RwLock<BidAskAggregator>>,
    pub sb_client: MyServiceBusClient,
    pub trading_executor: TradingExecutorGrpcClient,
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<SettingsReader>) -> Self {
        let connections = Arc::new(SignalrConnectionsList::new());

        let my_no_sql_connection = my_no_sql_tcp_reader::MyNoSqlTcpConnection::new(
            format!("{}", crate::app::APP_NAME),
            settings_reader.clone(),
        );

        let accounts_manager = AccountsManagerGrpcClient::new(settings_reader.clone());

        let sb_client = MyServiceBusClient::new(
            APP_NAME,
            APP_VERSION,
            settings_reader.clone(),
            my_logger::LOGGER.clone(),
        );

        let signalr_builder = Arc::new(SignalRPublshersBuilder::new(connections.clone()));
        Self {
            instruments_ns_reader: my_no_sql_connection.get_reader().await,
            sessions_ns_reader: my_no_sql_connection.get_reader().await,
            trading_groups_ns_reader: my_no_sql_connection.get_reader().await,
            trading_profile_ns_reader: my_no_sql_connection.get_reader().await,
            price_change_ns_reader: my_no_sql_connection.get_reader().await,
            bid_ask_snapshot_reader: my_no_sql_connection.get_reader().await,
            instruments_groups_ns_reader: my_no_sql_connection.get_reader().await,
            connections,
            accounts_manager,
            signalr_message_sender: Arc::new(SignalRMessageSender::new(&signalr_builder)),
            my_no_sql_connection,
            app_states: Arc::new(AppStates::create_initialized()),
            bid_ask_aggregator: Arc::new(RwLock::new(BidAskAggregator::new())),
            sb_client,
            trading_executor: TradingExecutorGrpcClient::new(settings_reader.clone()),
        }
    }
}
