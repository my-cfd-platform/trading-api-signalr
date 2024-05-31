use rest_api_wl_shared::middlewares::SessionEntity;
use service_sdk::{
    my_http_server::signal_r::MySignalRConnection, my_telemetry::MyTelemetryContext,
};

use crate::{
    keyvalue_grpc::GetKeyValueGrpcRequestModel, utils::init_signal_r_contract_now,
    AccountSignalRModel, AccountsSignalRModel, AppContext, SignalRConnectionContext, SignalRError,
    USER_ID_TAG,
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

    let mut accounts = super::get_client_accounts(app, &connection.ctx, telemetry).await?;

    let selected_account_id = app
        .key_value_grpc_client
        .get(
            vec![GetKeyValueGrpcRequestModel {
                client_id: session.trader_id.clone(),
                key: crate::SELECTED_ACCOUNT_ID_KEY.to_string(),
            }],
            telemetry,
        )
        .await;

    let selected_account_id = match selected_account_id {
        Ok(result) => result,
        Err(_) => None,
    };

    if let Some(selected_account_id) = selected_account_id {
        for selected_account_id_model in selected_account_id {
            if let Some(value) = selected_account_id_model.value.as_ref() {
                sort_accounts(&mut accounts, value);
            }
        }
    }

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

fn sort_accounts(accounts: &mut Vec<AccountSignalRModel>, selected_account_id: &str) {
    let index = accounts.iter().position(|x| x.id == selected_account_id);

    if let Some(index) = index {
        let selected_account = accounts.remove(index);
        accounts.insert(0, selected_account);
    }
}
