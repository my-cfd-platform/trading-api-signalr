use service_sdk::{my_http_server::signalr::MySignalrConnection, my_telemetry::MyTelemetryContext};

use crate::{
    get_account_data, get_client_account, get_trading_entities, get_trading_info, AppContext,
    SetActiveAccountCommand, SignalRConnectionContext, SignalRError, SignalRMessageWrapper,
    SignalRMessageWrapperWithAccount, SignalROutcomeMessage,
};

pub async fn process_set_active_account(
    app: &AppContext,
    message: SetActiveAccountCommand,
    connection: &MySignalrConnection<SignalRConnectionContext>,
    telemetry: &MyTelemetryContext,
) -> Result<(), SignalRError> {
    let (account, trading_group) =
        get_client_account(app, &connection.ctx, &message.account_id, telemetry).await?;

    let account_data = get_account_data(app, &account.id, &trading_group).await?;
    connection.ctx.set_active_account(account_data).await;

    let (instruments, groups, price_change) = get_trading_entities(app, &connection.ctx).await?;

    let active_positions = get_trading_info(app, &connection.ctx, telemetry).await?;

    app.signalr_message_sender
        .send_message(
            connection,
            SignalROutcomeMessage::Instruments(crate::SignalRMessageWrapperWithAccount::new(
                instruments,
                &account.id,
            )),
        )
        .await;

    app.signalr_message_sender
        .send_message(
            connection,
            SignalROutcomeMessage::InstrumentsGroups(crate::SignalRMessageWrapperWithAccount::new(
                groups,
                &account.id,
            )),
        )
        .await;

    app.signalr_message_sender
        .send_message(
            connection,
            SignalROutcomeMessage::PriceChange(SignalRMessageWrapper::new(price_change)),
        )
        .await;

    app.signalr_message_sender
        .send_message(
            connection,
            SignalROutcomeMessage::ActivePositions(SignalRMessageWrapperWithAccount::new(
                active_positions,
                &account.id,
            )),
        )
        .await;

    return Ok(());
}
