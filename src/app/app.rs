use std::sync::Arc;

use my_nosql_contracts::{
    BidAskSnapshotNoSqlEntity, PriceChangeSnapshotNoSqlEntity, TradingGroupNoSqlEntity,
    TradingInstrumentGroupNoSqlEntity, TradingInstrumentNoSqlEntity, TradingProfileNoSqlEntity,
};
use rest_api_wl_shared::middlewares::SessionEntity;
use service_sdk::{
    my_http_server::signalr::{SignalRPublshersBuilder, SignalrConnectionsList},
    my_no_sql_sdk::reader::MyNoSqlDataReader,
    ServiceContext,
};
use tokio::sync::RwLock;

use crate::{
    settings::SettingsReader, AccountsManagerGrpcClient, BidAskAggregator,
    SignalRConnectionContext, SignalRMessageSender, TradingExecutorGrpcClient,
};

pub struct AppContext {
    pub instruments_ns_reader:
        Arc<dyn MyNoSqlDataReader<TradingInstrumentNoSqlEntity> + Sync + Send>,
    pub bid_ask_snapshot_reader:
        Arc<dyn MyNoSqlDataReader<BidAskSnapshotNoSqlEntity> + Sync + Send>,
    pub sessions_ns_reader: Arc<dyn MyNoSqlDataReader<SessionEntity> + Sync + Send>,
    pub trading_groups_ns_reader: Arc<dyn MyNoSqlDataReader<TradingGroupNoSqlEntity> + Sync + Send>,
    pub price_change_ns_reader:
        Arc<dyn MyNoSqlDataReader<PriceChangeSnapshotNoSqlEntity> + Sync + Send>,
    pub trading_profile_ns_reader:
        Arc<dyn MyNoSqlDataReader<TradingProfileNoSqlEntity> + Sync + Send>,
    pub instruments_groups_ns_reader:
        Arc<dyn MyNoSqlDataReader<TradingInstrumentGroupNoSqlEntity> + Sync + Send>,
    pub connections: Arc<SignalrConnectionsList<SignalRConnectionContext>>,
    pub accounts_manager: AccountsManagerGrpcClient,
    pub signalr_message_sender: Arc<SignalRMessageSender>,
    pub bid_ask_aggregator: Arc<RwLock<BidAskAggregator>>,
    pub trading_executor: TradingExecutorGrpcClient,
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<SettingsReader>, sc: &ServiceContext) -> Self {
        let connections = Arc::new(SignalrConnectionsList::new());

        let accounts_manager =
            AccountsManagerGrpcClient::new(settings_reader.get_accounts_manager_grpc().await);
        let signalr_builder = Arc::new(SignalRPublshersBuilder::new(connections.clone()));

        Self {
            instruments_ns_reader: sc.get_ns_reader().await,
            sessions_ns_reader: sc.get_ns_reader().await,
            trading_groups_ns_reader: sc.get_ns_reader().await,
            trading_profile_ns_reader: sc.get_ns_reader().await,
            price_change_ns_reader: sc.get_ns_reader().await,
            bid_ask_snapshot_reader: sc.get_ns_reader().await,
            instruments_groups_ns_reader: sc.get_ns_reader().await,
            connections,
            accounts_manager,
            signalr_message_sender: Arc::new(SignalRMessageSender::new(&signalr_builder)),
            bid_ask_aggregator: Arc::new(RwLock::new(BidAskAggregator::new())),
            trading_executor: TradingExecutorGrpcClient::new(
                settings_reader.get_trading_executor_grpc().await,
            ),
        }
    }
}
