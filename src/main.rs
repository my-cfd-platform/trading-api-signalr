use std::sync::Arc;
use trading_api_signalr::{setup_server, AppContext, SettingsReader};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".my-cfd").await;
    let settings_reader = Arc::new(settings_reader.get_settings().await);

    let app = Arc::new(AppContext::new(settings_reader.clone()).await);

    setup_server(app.clone());
    app.my_no_sql_connection.start(my_logger::LOGGER.clone()).await;

    app.app_states.wait_until_shutdown().await;
}
