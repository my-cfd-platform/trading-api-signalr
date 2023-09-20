use service_sdk::{my_http_server::signal_r::MySignalRMiddleware, ServiceContext};
use std::sync::Arc;

use crate::{
    action_subscribers::{
        InitSignalRActionSubscriber, PingSignalRActionSubscriber,
        SetActiveAccountSignalRActionSubscriber,
    },
    app::AppContext,
    SignalRConnectionContext,
};

pub fn setup_signal_r(app: Arc<AppContext>, sc: &mut ServiceContext) {
    sc.configure_http_server(|x| {
        let signal_r_middleware: MySignalRMiddleware<SignalRConnectionContext> =
            MySignalRMiddleware::new_with_builder(
                "signalr",
                app.connections.clone(),
                service_sdk::my_logger::LOGGER.clone(),
            )
            .with_action(InitSignalRActionSubscriber::new(app.clone()))
            .with_action(SetActiveAccountSignalRActionSubscriber::new(app.clone()))
            .with_action(PingSignalRActionSubscriber::new(app.clone()))
            .build();
        x.register_custom_middleware(Arc::new(signal_r_middleware))
    });
}
