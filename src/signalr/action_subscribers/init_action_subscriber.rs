use std::collections::HashMap;
use std::sync::Arc;

service_sdk::macros::use_signal_r_subscriber!();

use crate::{AppContext, InitSignalRModel, SignalRConnectionContext};

pub struct InitSignalRActionSubscriber {
    app: Arc<AppContext>,
}

impl InitSignalRActionSubscriber {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl MySignalRActionSubscriber<InitSignalRModel> for InitSignalRActionSubscriber {
    type TCtx = SignalRConnectionContext;

    async fn on(
        &self,
        connection: &Arc<MySignalRConnection<SignalRConnectionContext>>,
        _: Option<HashMap<String, String>>,
        model: InitSignalRModel,
        my_telemetry: &mut SignalRTelemetry,
    ) {
        println!("Session token: {}", model.session_token);
        let result = crate::flows::process_init(
            &self.app,
            &model.session_token,
            connection,
            my_telemetry.get_ctx(),
        )
        .await;

        if let Err(err) = result {
            self.app
                .signal_r_message_sender
                .error_publisher
                .send_to_connection(connection, err.get_message())
                .await;
        }
    }
}
