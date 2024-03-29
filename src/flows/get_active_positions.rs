use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{
    trading_executor_grpc::TradingExecutorGetActivePositionsGrpcRequest,
    ActivePositionSignalRModel, AppContext, SignalRConnectionContext, SignalRError,
};

pub async fn get_active_positions(
    app: &AppContext,
    signal_r_ctx: &SignalRConnectionContext,
    telemetry: &MyTelemetryContext,
) -> Result<Vec<ActivePositionSignalRModel>, SignalRError> {
    let active_positions = match app
        .trading_executor
        .get_account_active_positions(
            TradingExecutorGetActivePositionsGrpcRequest {
                trader_id: signal_r_ctx.get_trader_id().await?,
                account_id: signal_r_ctx.get_account_data().await?.account_id,
            },
            &telemetry,
        )
        .await
    {
        Ok(positions) => Ok(positions),
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(SignalRError::NetworkError(
                "Grpc error. TradingExecutor".to_string(),
            ));
        }
    }?;

    let active_positions = match active_positions {
        Some(src) => src,
        None => vec![],
    }
    .iter()
    .map(|x| x.to_owned().into())
    .collect();

    return Ok(active_positions);
}
