use service_sdk::{my_http_server::signalr::MySignalrMiddleware, ServiceContext};
use std::sync::Arc;

use crate::{
    app::AppContext, SignalRConnectionContext, SignalRInitMessageProcessor,
    SignalRPingMessageProcessor, SignalRSetActiveAccountMessageProcessor,
};

pub fn setup_signal_r(app: Arc<AppContext>, sc: &mut ServiceContext) {
    sc.configure_http_server(|x| {
        let signalr_middleware: MySignalrMiddleware<SignalRConnectionContext> =
            MySignalrMiddleware::new_with_builder(
                "signalr",
                app.connections.clone(),
                service_sdk::my_logger::LOGGER.clone(),
            )
            .with_action(
                "init".to_string(),
                SignalRInitMessageProcessor::new(app.clone()),
            )
            .with_action(
                "SetActiveAccount".to_string(),
                SignalRSetActiveAccountMessageProcessor::new(app.clone()),
            )
            .with_action(
                "ping".to_string(),
                SignalRPingMessageProcessor::new(app.clone()),
            )
            .build();
        x.register_custom_middleware(Arc::new(signalr_middleware))
    });
}
