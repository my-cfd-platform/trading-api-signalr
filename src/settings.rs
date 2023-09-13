use my_no_sql_tcp_reader::MyNoSqlTcpConnectionSettings;
use my_service_bus_tcp_client::MyServiceBusSettings;
use my_settings_reader::SettingsModel;
use serde::{Deserialize, Serialize};

use crate::{AccountsManagerGrpcClient, TradingExecutorGrpcClient};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "AccountsManagerGrpc")]
    pub accounts_manager_grpc: String,
    #[serde(rename = "NoSqlTcp")]
    pub nosql_tcp: String,
    #[serde(rename = "SbTcp")]
    pub sb_tcp: String,
    #[serde(rename = "TradingExecutorGrpcUrl")]
    pub trading_executor_url: String,
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
impl my_grpc_extensions::GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == AccountsManagerGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.accounts_manager_grpc.clone();
        }

        if name == TradingExecutorGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.trading_executor_url.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
