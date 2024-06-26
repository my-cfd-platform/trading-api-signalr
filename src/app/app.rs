use std::sync::Arc;

use my_nosql_contracts::*;
use rest_api_wl_shared::middlewares::SessionEntity;
use service_sdk::{
    my_http_server::signal_r::{SignalRConnectionsList, SignalRPublishersBuilder},
    my_no_sql_sdk::reader::MyNoSqlDataReaderTcp,
    ServiceContext,
};
use tokio::sync::Mutex;

use crate::{
    settings::SettingsReader, AccountsManagerGrpcClient, BidAskAggregator, KeyValueGrpcClient,
    SignalRConnectionContext, SignalRMessageSender, TradingExecutorGrpcClient,
};

pub struct AppContext {
    pub instruments_ns_reader: Arc<MyNoSqlDataReaderTcp<TradingInstrumentNoSqlEntity>>,
    pub bid_ask_snapshot_reader: Arc<MyNoSqlDataReaderTcp<BidAskSnapshotNoSqlEntity>>,
    pub sessions_ns_reader: Arc<MyNoSqlDataReaderTcp<SessionEntity>>,
    pub trading_groups_ns_reader: Arc<MyNoSqlDataReaderTcp<TradingGroupNoSqlEntity>>,
    pub price_change_ns_reader: Arc<MyNoSqlDataReaderTcp<PriceChangeSnapshotNoSqlEntity>>,
    pub trading_profile_ns_reader: Arc<MyNoSqlDataReaderTcp<TradingProfileNoSqlEntity>>,
    pub instruments_groups_ns_reader: Arc<MyNoSqlDataReaderTcp<TradingInstrumentGroupNoSqlEntity>>,
    pub markup_profiles_ns_reader: Arc<MyNoSqlDataReaderTcp<MarkupProfileNoSqlEntity>>,
    pub connections: Arc<SignalRConnectionsList<SignalRConnectionContext>>,
    pub accounts_manager: AccountsManagerGrpcClient,
    pub signal_r_message_sender: Arc<SignalRMessageSender>,
    pub bid_ask_aggregator: Arc<Mutex<BidAskAggregator>>,
    pub trading_executor: TradingExecutorGrpcClient,
    pub key_value_grpc_client: KeyValueGrpcClient,
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<SettingsReader>, sc: &ServiceContext) -> Self {
        let connections = Arc::new(SignalRConnectionsList::new());

        let accounts_manager =
            AccountsManagerGrpcClient::new(settings_reader.get_accounts_manager_grpc().await);
        let signal_r_builder = Arc::new(SignalRPublishersBuilder::new(connections.clone()));

        Self {
            instruments_ns_reader: sc.get_ns_reader().await,
            sessions_ns_reader: sc.get_ns_reader().await,
            trading_groups_ns_reader: sc.get_ns_reader().await,
            trading_profile_ns_reader: sc.get_ns_reader().await,
            price_change_ns_reader: sc.get_ns_reader().await,
            bid_ask_snapshot_reader: sc.get_ns_reader().await,
            instruments_groups_ns_reader: sc.get_ns_reader().await,
            markup_profiles_ns_reader: sc.get_ns_reader().await,
            connections,
            accounts_manager,
            key_value_grpc_client: KeyValueGrpcClient::new(settings_reader.clone()),
            signal_r_message_sender: Arc::new(SignalRMessageSender::new(&signal_r_builder)),
            bid_ask_aggregator: Arc::new(Mutex::new(BidAskAggregator::new())),
            trading_executor: TradingExecutorGrpcClient::new(
                settings_reader.get_trading_executor_grpc().await,
            ),
        }
    }
}

#[service_sdk::async_trait::async_trait]
impl fx_utils::MarkupUtils for AppContext {
    async fn get_instrument(
        &self,
        instrument_id: &str,
    ) -> Option<Arc<TradingInstrumentNoSqlEntity>> {
        self.instruments_ns_reader
            .get_entity(
                TradingInstrumentNoSqlEntity::generate_partition_key(),
                instrument_id,
            )
            .await
    }
    async fn get_trading_group(&self, group_id: &str) -> Option<Arc<TradingGroupNoSqlEntity>> {
        self.trading_groups_ns_reader
            .get_entity(TradingGroupNoSqlEntity::generate_partition_key(), group_id)
            .await
    }

    async fn get_markup_profile(&self, profile_id: &str) -> Option<Arc<MarkupProfileNoSqlEntity>> {
        self.markup_profiles_ns_reader
            .get_entity(
                MarkupProfileNoSqlEntity::generate_partition_key(),
                profile_id,
            )
            .await
    }
}
