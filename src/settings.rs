use std::sync::Arc;

use serde::{Deserialize, Serialize};
use service_sdk::{async_trait, my_grpc_extensions::GrpcClientSettings};

service_sdk::macros::use_settings!();

#[derive(
    my_settings_reader::SettingsModel, SdkSettingsTraits, Serialize, Deserialize, Debug, Clone,
)]
pub struct SettingsModel {
    pub accounts_manager_grpc: String,
    pub nosql_tcp: String,
    pub sb_tcp: String,
    pub trading_executor_url: String,
    pub my_telemetry: String,
    pub seq_conn_string: String,
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
}

pub struct GrpcUrl(String);

impl GrpcUrl {
    pub fn new(url: String) -> Self {
        Self(url)
    }
}

#[async_trait::async_trait]
impl SeqSettings for SettingsReader {
    async fn get_conn_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.seq_conn_string.clone()
    }
}

#[async_trait::async_trait]
impl GrpcClientSettings for GrpcUrl {
    async fn get_grpc_url(&self, _: &'static str) -> String {
        self.0.clone()
    }
}

#[async_trait::async_trait]
impl MyNoSqlTcpConnectionSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.nosql_tcp.clone()
    }
}

#[async_trait::async_trait]
impl MyServiceBusSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.sb_tcp.clone()
    }
}

#[async_trait::async_trait]
impl MyTelemetrySettings for SettingsReader {
    async fn get_telemetry_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.my_telemetry.clone()
    }
}
