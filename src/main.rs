mod app;
mod bg;
mod flows;
mod grpc_clients;

mod sb_listeners;
mod services;
mod settings;

mod signalr;

const SELECTED_ACCOUNT_ID_KEY: &'static str = ".selected-account-id";

pub mod accounts_manager_grpc {
    tonic::include_proto!("accounts_manager");
}
pub mod trading_executor_grpc {
    tonic::include_proto!("trading_executor");
}

pub mod keyvalue_grpc {
    tonic::include_proto!("keyvalue");
}

pub use app::*;
pub use bg::*;
pub use grpc_clients::*;
use service_sdk::ServiceInfo;
pub use services::*;
pub use signalr::*;

use crate::sb_listeners::PricesListener;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let settings_reader = settings::SettingsReader::new(".my-cfd-platform").await;
    let settings_reader = Arc::new(settings_reader);

    let quotes_tick_freq = settings_reader.get_quotes_tick_freq().await;

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;

    let app_context = Arc::new(AppContext::new(&settings_reader, &service_context).await);

    service_context.register_sb_subscribe(
        Arc::new(PricesListener::new(app_context.clone())),
        service_sdk::my_service_bus::abstractions::subscriber::TopicQueueType::DeleteOnDisconnect,
    ).await;

    service_context.register_sb_subscribe(
        Arc::new(AccountsUpdatesListener::new(app_context.clone())),
        service_sdk::my_service_bus::abstractions::subscriber::TopicQueueType::DeleteOnDisconnect,
    ).await;

    service_context.register_sb_subscribe(
        Arc::new(PositionsUpdateListener::new(app_context.clone())),
        service_sdk::my_service_bus::abstractions::subscriber::TopicQueueType::DeleteOnDisconnect,
    ).await;

    service_context.register_sb_subscribe(
        Arc::new(PendingPositionsUpdateListener::new(app_context.clone())),
        service_sdk::my_service_bus::abstractions::subscriber::TopicQueueType::DeleteOnDisconnect,
    ).await;

    service_context.register_timer(std::time::Duration::from_millis(quotes_tick_freq), |x| {
        x.register_timer(
            "prices-sender",
            Arc::new(PriceSendTimer::new(app_context.clone())),
        )
    });
    trade_log::core::TRADE_LOG
        .init_component_name(settings_reader.get_service_name().as_str())
        .await;
    trade_log::core::TRADE_LOG
        .start(&service_context.sb_client)
        .await;
    setup_signal_r(app_context.clone(), &mut service_context);

    service_context.start_application().await;
}
