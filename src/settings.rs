use std::sync::Arc;

use serde::{Deserialize, Serialize};
use service_sdk::async_trait;

use crate::KeyValueGrpcClient;

service_sdk::macros::use_settings!();

#[derive(
    my_settings_reader::SettingsModel,
    SdkSettingsTraits,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    AutoGenerateSettingsTraits,
)]
pub struct SettingsModel {
    pub accounts_manager_grpc: String,
    pub my_no_sql_tcp_reader: String,
    pub my_sb_tcp_host_port: String,
    pub trading_executor_url: String,
    pub my_telemetry: String,
    pub seq_conn_string: String,
    pub key_value_grpc_url: String,
    pub quotes_tick_freq_millis: u64,
}

impl SettingsReader {
    pub async fn get_accounts_manager_grpc(&self) -> Arc<GrpcUrl> {
        let read_access = self.settings.read().await;
        return Arc::new(GrpcUrl::new(read_access.accounts_manager_grpc.clone()));
    }

    pub async fn get_trading_executor_grpc(&self) -> Arc<GrpcUrl> {
        let read_access = self.settings.read().await;
        return Arc::new(GrpcUrl::new(read_access.trading_executor_url.clone()));
    }

    pub async fn get_quotes_tick_freq(&self) -> u64 {
        let read_access = self.settings.read().await;
        return read_access.quotes_tick_freq_millis;
    }
}

pub struct GrpcUrl(String);

impl GrpcUrl {
    pub fn new(url: String) -> Self {
        Self(url)
    }
}

#[async_trait::async_trait]
impl GrpcClientSettings for GrpcUrl {
    async fn get_grpc_url(&self, _: &'static str) -> String {
        self.0.clone()
    }
}

#[async_trait::async_trait]
impl GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == KeyValueGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.key_value_grpc_url.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
