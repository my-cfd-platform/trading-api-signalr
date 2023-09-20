use service_sdk::{
    my_http_server::signal_r::MySignalRConnection, my_telemetry::MyTelemetryContext,
};

use crate::{
    utils::init_signal_r_contract_now, ActivePositionsSignalRModel, AppContext,
    InstrumentGroupsSignalRModel, InstrumentsSignalRModel, PriceChangesSignalRModel,
    SignalRConnectionContext, SignalRError,
};

pub async fn set_active_account(
    app: &AppContext,
    account_id: String,
    connection: &MySignalRConnection<SignalRConnectionContext>,
    telemetry: &MyTelemetryContext,
) -> Result<(), SignalRError> {
    let (account, trading_group) =
        super::get_client_account(app, &connection.ctx, &account_id, telemetry).await?;

    let account_data = super::get_account_data(app, &account.id, &trading_group).await?;
    connection.ctx.set_active_account(account_data).await;

    let (instruments, groups, price_change) =
        super::get_trading_entities(app, &connection.ctx).await?;

    let active_positions = super::get_trading_info(app, &connection.ctx, telemetry).await?;

    app.signal_r_message_sender
        .instruments_publisher
        .send_to_connection(
            connection,
            InstrumentsSignalRModel {
                now: init_signal_r_contract_now(),
                data: instruments,
                account_id: account.id.to_string(),
            },
        )
        .await;

    app.signal_r_message_sender
        .instruments_groups_publisher
        .send_to_connection(
            connection,
            InstrumentGroupsSignalRModel {
                now: init_signal_r_contract_now(),
                data: groups,
                account_id: account.id.to_string(),
            },
        )
        .await;

    app.signal_r_message_sender
        .price_change_publisher
        .send_to_connection(
            connection,
            PriceChangesSignalRModel {
                now: init_signal_r_contract_now(),
                data: price_change,
            },
        )
        .await;

    app.signal_r_message_sender
        .active_position_publisher
        .send_to_connection(
            connection,
            ActivePositionsSignalRModel {
                now: init_signal_r_contract_now(),
                data: active_positions,
                account_id: account.id.to_string(),
            },
        )
        .await;

    return Ok(());
}
