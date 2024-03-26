use rest_api_wl_shared::middlewares::SessionEntity;
use service_sdk::{
    my_http_server::signal_r::MySignalRConnection, my_telemetry::MyTelemetryContext,
};

use crate::{
    utils::init_signal_r_contract_now, AccountsSignalRModel, AppContext, SignalRConnectionContext,
    SignalRError, USER_ID_TAG,
};

pub async fn process_init(
    app: &AppContext,
    session_token: &str,
    connection: &MySignalRConnection<SignalRConnectionContext>,
    telemetry: &MyTelemetryContext,
) -> Result<(), SignalRError> {
    let session = app
        .sessions_ns_reader
        .get_entity(&SessionEntity::get_pk(), session_token)
        .await;

    let Some(session) = session else {
        return Err(SignalRError::SessionNotFound);
    };

    connection.ctx.set_trader_id(&session.trader_id).await;

    app.connections
        .add_tag_to_connection(connection, USER_ID_TAG, &session.trader_id)
        .await;

    let accounts = super::get_client_accounts(app, &connection.ctx, telemetry).await?;

    trade_log::trade_log!(
        &session.trader_id,
        "",
        "",
        "",
        "Initialized signal-R init.",
        telemetry.clone(),
        "accounts" = &accounts
    );

    app.signal_r_message_sender
        .accounts_publisher
        .send_to_connection(
            connection,
            &AccountsSignalRModel {
                now: init_signal_r_contract_now(),
                data: accounts,
            },
        )
        .await;

    return Ok(());
}
