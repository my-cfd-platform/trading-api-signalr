use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;
use my_signalr_middleware::MySignalrMiddleware;

use crate::{app::AppContext, SignalRInitMessageProcessor, SignalRPingMessageProcessor, SignalRConnectionContext};

pub fn setup_server(app: Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8000)));

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(
        crate::app::APP_NAME.to_string(),
        crate::app::APP_VERSION.to_string(),
    )));

    let signalr_middleware: MySignalrMiddleware<SignalRConnectionContext> = MySignalrMiddleware::new_with_builder(
        "signalr",
        app.connections.clone(),
        my_logger::LOGGER.clone(),
    )
    .with_action(
        "init".to_string(),
        SignalRInitMessageProcessor::new(app.clone()),
    )
    .with_action(
        "ping".to_string(),
        SignalRPingMessageProcessor::new(app.clone()),
    )
    .build();

    http_server.add_middleware(Arc::new(signalr_middleware));

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
