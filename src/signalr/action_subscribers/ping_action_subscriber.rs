use std::collections::HashMap;
use std::sync::Arc;

service_sdk::macros::use_signal_r_subscriber!();

use crate::{
    utils::init_signal_r_contract_now, AppContext, PingSignalRModel, PongSignalRModel,
    SignalRConnectionContext,
};

pub struct PingSignalRActionSubscriber {
    app: Arc<AppContext>,
}

impl PingSignalRActionSubscriber {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl MySignalRActionSubscriber<PingSignalRModel> for PingSignalRActionSubscriber {
    type TCtx = SignalRConnectionContext;

    async fn on(
        &self,
        connection: &Arc<MySignalRConnection<SignalRConnectionContext>>,
        _: Option<HashMap<String, String>>,
        _: PingSignalRModel,
        my_telemetry: &mut SignalRTelemetry,
    ) {
        self.app
            .signal_r_message_sender
            .pong_publisher
            .send_to_connection(
                connection,
                PongSignalRModel {
                    now: init_signal_r_contract_now(),
                },
            )
            .await;

        my_telemetry.do_not_track_this_event();
    }
}
