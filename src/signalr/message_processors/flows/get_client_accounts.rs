use my_nosql_contracts::{TradingGroupNoSqlEntity, TradingProfileNoSqlEntity};
use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{
    accounts_manager_grpc::{
        AccountManagerGetClientAccountGrpcRequest, AccountManagerGetClientAccountsGrpcRequest,
    },
    AccountData, AccountSignalRModel, AppContext, SignalRConnectionContext,
    SignalRError,
};

pub async fn get_client_accounts(
    app: &AppContext,
    signal_r_ctx: &SignalRConnectionContext,
    telemetry: &MyTelemetryContext,
) -> Result<Vec<AccountSignalRModel>, SignalRError> {
    let Ok(accounts) = app
        .accounts_manager
        .get_client_accounts(
            AccountManagerGetClientAccountsGrpcRequest {
                trader_id: signal_r_ctx.get_trader_id().await?,
            },
            &telemetry,
        )
        .await
    else {
        return Err(SignalRError::NetworkError("Grpc error".to_string()));
    };

    let Some(accounts) = accounts else {
        return Err(SignalRError::AccountsNotFound);
    };

    return Ok(accounts.iter().map(|acc| acc.to_owned().into()).collect());
}

pub async fn get_client_account(
    app: &AppContext,
    signal_r_ctx: &SignalRConnectionContext,
    account_id: &str,
    telemetry: &MyTelemetryContext,
) -> Result<(AccountSignalRModel, String), SignalRError> {

    let Ok(account) = app
        .accounts_manager
        .get_client_account(
            AccountManagerGetClientAccountGrpcRequest {
                trader_id: signal_r_ctx.get_trader_id().await?,
                account_id: account_id.to_string(),
            },
            &telemetry,
        )
        .await
    else {
        return Err(SignalRError::NetworkError(
            "Grpc error: AccountsManager".to_string(),
        ));
    };

    let Some(account) = account.account else {
        return Err(SignalRError::AccountsNotFound);
    };

    if account.trading_disabled {
        return Err(SignalRError::TradingDisabled);
    }

    return Ok((account.clone().into(), account.trading_group.clone()));
}

pub async fn get_account_data(
    app: &AppContext,
    account_id: &str,
    account_trading_group: &str,
) -> Result<AccountData, SignalRError> {
    let Some(trading_group) = app
        .trading_groups_ns_reader
        .get_entity(
            TradingGroupNoSqlEntity::generate_partition_key(),
            account_trading_group,
        )
        .await
    else {
        return Err(SignalRError::TradingGroupNotFound);
    };

    let Some(trading_profile) = app
        .trading_profile_ns_reader
        .get_entity(
            TradingProfileNoSqlEntity::generate_partition_key(),
            &trading_group.trading_profile_id,
        )
        .await
    else {
        return Err(SignalRError::TradingProfileNotFound);
    };

    return Ok(AccountData {
        account_id: account_id.to_string(),
        trading_group_id: trading_group.id.clone(),
        trading_profile_id: trading_profile.id.clone(),
    });
}
