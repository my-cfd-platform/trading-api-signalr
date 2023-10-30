use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{
    trading_executor_grpc::TradingExecutorGetAccountPendingPositionGrpcRequest, AppContext,
    PendingPositionSignalRModel, SignalRConnectionContext, SignalRError,
};

pub async fn get_pending_positions(
    app: &AppContext,
    signal_r_ctx: &SignalRConnectionContext,
    telemetry: &MyTelemetryContext,
) -> Result<Vec<PendingPositionSignalRModel>, SignalRError> {
    let Ok(active_positions) = app
        .trading_executor
        .get_account_pending_positions(
            TradingExecutorGetAccountPendingPositionGrpcRequest {
                trader_id: signal_r_ctx.get_trader_id().await?,
                account_id: signal_r_ctx.get_account_data().await?.account_id,
            },
            &telemetry,
        )
        .await
    else {
        return Err(SignalRError::NetworkError(
            "Grpc error. TradingExecutor".to_string(),
        ));
    };

    let active_positions = match active_positions {
        Some(src) => src,
        None => vec![],
    }
    .iter()
    .map(|x| x.to_owned().into())
    .collect();

    return Ok(active_positions);
}
