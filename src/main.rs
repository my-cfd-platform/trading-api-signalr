use my_seq_logger::SeqLogger;
use rust_extensions::MyTimer;
use std::sync::Arc;
use trading_api_signalr::{
    settings::SettingsReader, setup_server, AccountsUpdatesListener, AppContext,
    PositionsUpdateListener, PriceSendTimer, PricesListener, APP_NAME, APP_VERSION,
};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".my-cfd").await;
    let settings_reader = Arc::new(settings_reader);
    let mut send_prices_timer = MyTimer::new(std::time::Duration::from_millis(300));

    let app = Arc::new(AppContext::new(&settings_reader).await);

    SeqLogger::enable_from_connection_string(settings_reader.clone());

    my_logger::LOGGER
        .populate_app_and_version(APP_NAME, APP_VERSION)
        .await;

    let telemetry_writer =
        my_telemetry_writer::MyTelemetryWriter::new(APP_NAME.to_string(), settings_reader.clone());

    send_prices_timer.register_timer("send prices", Arc::new(PriceSendTimer::new(app.clone())));

    app.sb_client
        .subscribe(
            APP_NAME.to_string(),
            my_service_bus_abstractions::subscriber::TopicQueueType::DeleteOnDisconnect,
            Arc::new(PricesListener::new(app.clone())),
        )
        .await;

    app.sb_client
        .subscribe(
            APP_NAME.to_string(),
            my_service_bus_abstractions::subscriber::TopicQueueType::DeleteOnDisconnect,
            Arc::new(AccountsUpdatesListener::new(app.clone())),
        )
        .await;

    app.sb_client
        .subscribe(
            APP_NAME.to_string(),
            my_service_bus_abstractions::subscriber::TopicQueueType::DeleteOnDisconnect,
            Arc::new(PositionsUpdateListener::new(app.clone())),
        )
        .await;

    send_prices_timer.start(app.app_states.clone(), my_logger::LOGGER.clone());
    app.sb_client.start().await;
    telemetry_writer.start(app.app_states.clone(), my_logger::LOGGER.clone());
    setup_server(app.clone());
    app.my_no_sql_connection
        .start(my_logger::LOGGER.clone())
        .await;

    app.app_states.wait_until_shutdown().await;
}
