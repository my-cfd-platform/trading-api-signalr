use my_no_sql_tcp_reader::MyNoSqlTcpConnectionSettings;
use my_service_bus_tcp_client::MyServiceBusSettings;
use my_settings_reader::SettingsModel;
use serde::{Deserialize, Serialize};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "AccountsManagerGrpc")]
    pub accounts_manager_grpc: String,
    #[serde(rename = "NoSqlTcp")]
    pub nosql_tcp: String,
    #[serde(rename = "SbTcp")]
    pub sb_tcp: String,
}

#[async_trait::async_trait]
impl MyNoSqlTcpConnectionSettings for SettingsModel {
    async fn get_host_port(&self) -> String {
        self.nosql_tcp.clone()
    }
}

#[async_trait::async_trait]
impl MyServiceBusSettings for SettingsModel {
    async fn get_host_port(&self) -> String {
        self.nosql_tcp.clone()
    }
}
