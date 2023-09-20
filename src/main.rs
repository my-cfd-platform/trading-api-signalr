use std::sync::Arc;

use trading_api_signalr::{
    setup_signal_r, AccountsUpdatesListener, AppContext, PositionsUpdateListener, PriceSendTimer,
    PricesListener, SettingsReader,
};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".my-cfd-platform").await;
    let settings_reader = Arc::new(settings_reader);

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

    service_context.register_background_job(
        std::time::Duration::from_millis(300),
        "prices-sender",
        Arc::new(PriceSendTimer::new(app_context.clone())),
    );

    setup_signal_r(app_context.clone(), &mut service_context);

    service_context.start_application().await;
}
