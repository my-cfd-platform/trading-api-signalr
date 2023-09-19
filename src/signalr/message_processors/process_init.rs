use rest_api_wl_shared::middlewares::SessionEntity;
use service_sdk::{my_http_server::signalr::MySignalrConnection, my_telemetry::MyTelemetryContext};

use crate::{
    AppContext, SignalRConnectionContext, SignalRError, SignalRInitAction, SignalRMessageWrapper,
    SignalROutcomeMessage, USER_ID_TAG,
};

pub async fn process_init(
    app: &AppContext,
    message: SignalRInitAction,
    connection: &MySignalrConnection<SignalRConnectionContext>,
    telemetry: &MyTelemetryContext,
) -> Result<(), SignalRError> {
    let my_telemetry = MyTelemetryContext::new();
    let _ = my_telemetry.start_event_tracking("SignalR Init");

    let session = app
        .sessions_ns_reader
        .get_entity(&SessionEntity::get_pk(), &message.token)
        .await;

    let Some(session) = session else {
        return Err(SignalRError::SessionNotFound);
    };

    connection.ctx.set_trader_id(&session.trader_id).await;

    app.connections
        .add_tag_to_connection(connection, USER_ID_TAG, &session.trader_id)
        .await;

    let accounts = crate::get_client_accounts(app, &connection.ctx, telemetry).await?;

    app.signalr_message_sender
        .send_message(
            connection,
            SignalROutcomeMessage::Accounts(SignalRMessageWrapper::new(accounts)),
        )
        .await;

    return Ok(());
}
