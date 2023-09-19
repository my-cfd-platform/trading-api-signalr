use std::sync::Arc;

use service_sdk::{my_http_server::signalr::MySignalrConnection, my_telemetry::MyTelemetryContext};

use crate::{
    AppContext, SignalRConnectionContext, SignalRError, SignalRIncomeMessage,
    SignalRMessageWrapperEmpty, SignalROutcomeMessage,
};

pub async fn handle_signal_r_message(
    app: &Arc<AppContext>,
    message: SignalRIncomeMessage,
    connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
) {
    match handle_message(app, message, connection).await {
        Ok(_) => {}
        Err(err) => {
            app.signalr_message_sender
                .send_message(connection, SignalROutcomeMessage::Error(err.get_message()))
                .await
        }
    }
}

async fn handle_message(
    app: &Arc<AppContext>,
    message: SignalRIncomeMessage,
    connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
) -> Result<(), SignalRError> {
    match message {
        SignalRIncomeMessage::Init(message) => {
            let my_telemetry = MyTelemetryContext::new();
            let _ = my_telemetry.start_event_tracking("SignalR Init");

            crate::process_init(&app, message, &connection, &my_telemetry).await?;
        }
        SignalRIncomeMessage::SetActiveAccount(set_account_message) => {
            let my_telemetry = MyTelemetryContext::new();

            let _ = my_telemetry.start_event_tracking("SetActiveAccount");
            crate::process_set_active_account(app, set_account_message, connection, &my_telemetry)
                .await?;
        }
        SignalRIncomeMessage::Ping => {
            app.signalr_message_sender
                .send_message(
                    connection,
                    SignalROutcomeMessage::Pong(SignalRMessageWrapperEmpty::new()),
                )
                .await
        }
    };

    return Ok(());
}
