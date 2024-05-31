use std::sync::Arc;

use service_sdk::{
    my_http_server::signal_r::MySignalRConnection, my_telemetry::MyTelemetryContext,
};

use crate::{
    keyvalue_grpc::SetKeyValueGrpcRequestModel, utils::init_signal_r_contract_now,
    ActivePositionsSignalRModel, AppContext, InstrumentGroupsSignalRModel, InstrumentsSignalRModel,
    PendingPositionsSignalRModel, PriceChangesSignalRModel, SignalRConnectionContext, SignalRError,
};

pub async fn set_active_account(
    app: &Arc<AppContext>,
    account_id: String,
    connection: &MySignalRConnection<SignalRConnectionContext>,
    telemetry: &MyTelemetryContext,
) -> Result<(), SignalRError> {
    let trader_id = connection.ctx.get_trader_id().await?;

    let app_spawned = app.clone();

    let telemetry_spawned = telemetry.clone();

    let trader_id_spawned = trader_id.clone();
    let account_id_spawned = account_id.clone();
    tokio::spawn(async move {
        let _ = app_spawned
            .key_value_grpc_client
            .set(
                vec![SetKeyValueGrpcRequestModel {
                    client_id: trader_id_spawned,
                    key: crate::SELECTED_ACCOUNT_ID_KEY.to_string(),
                    value: account_id_spawned,
                }],
                &telemetry_spawned,
            )
            .await;
    });

    let (account, trading_group) =
        super::get_client_account(app, &connection.ctx, &account_id, telemetry).await?;

    let account_data = super::get_account_data(app, &account.id, &trading_group).await?;
    connection
        .ctx
        .set_active_account(account_data.clone())
        .await;

    let (instruments, groups, price_change) =
        super::get_trading_entities(app, &connection.ctx).await?;

    let active_positions = super::get_active_positions(app, &connection.ctx, telemetry).await?;
    let pending_positions = super::get_pending_positions(app, &connection.ctx, telemetry).await?;

    app.signal_r_message_sender
        .instruments_publisher
        .send_to_connection(
            connection,
            &InstrumentsSignalRModel {
                now: init_signal_r_contract_now(),
                data: instruments.clone(),
                account_id: account.id.to_string(),
            },
        )
        .await;

    app.signal_r_message_sender
        .instruments_groups_publisher
        .send_to_connection(
            connection,
            &InstrumentGroupsSignalRModel {
                now: init_signal_r_contract_now(),
                data: groups.clone(),
                account_id: account.id.to_string(),
            },
        )
        .await;

    app.signal_r_message_sender
        .price_change_publisher
        .send_to_connection(
            connection,
            &PriceChangesSignalRModel {
                now: init_signal_r_contract_now(),
                data: price_change.clone(),
            },
        )
        .await;

    app.signal_r_message_sender
        .active_position_publisher
        .send_to_connection(
            connection,
            &ActivePositionsSignalRModel {
                now: init_signal_r_contract_now(),
                data: active_positions.clone(),
                account_id: account.id.to_string(),
            },
        )
        .await;

    app.signal_r_message_sender
        .pending_position_publisher
        .send_to_connection(
            connection,
            &PendingPositionsSignalRModel {
                now: init_signal_r_contract_now(),
                data: pending_positions.clone(),
                account_id: account.id.to_string(),
            },
        )
        .await;

    trade_log::trade_log!(
        &trader_id,
        &account.id,
        "",
        "",
        "Processed set account.",
        telemetry.clone(),
        "account" = &account,
        "account_data" = &account_data,
        "instruments" = &instruments,
        "groups" = &groups,
        "price_change" = &price_change,
        "active_positions" = &active_positions,
        "pending_positions" = &pending_positions
    );
    return Ok(());
}
