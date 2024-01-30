use std::collections::HashMap;
use std::sync::Arc;

service_sdk::macros::use_signal_r_subscriber!();

use crate::{AppContext, SetActiveAccountModel, SignalRConnectionContext};

pub struct SetActiveAccountSignalRActionSubscriber {
    app: Arc<AppContext>,
}

impl SetActiveAccountSignalRActionSubscriber {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl MySignalRActionSubscriber<SetActiveAccountModel> for SetActiveAccountSignalRActionSubscriber {
    type TCtx = SignalRConnectionContext;

    async fn on(
        &self,
        connection: &Arc<MySignalRConnection<SignalRConnectionContext>>,
        _: Option<HashMap<String, String>>,
        model: SetActiveAccountModel,
        ctx: &mut SignalRTelemetry,
    ) {
        let result = crate::flows::set_active_account(
            &self.app,
            model.account_id.clone(),
            &connection,
            ctx.get_ctx(),
        )
        .await;

        if let Err(err) = result {
            if let Ok(trader_id) = connection.ctx.get_trader_id().await {
                trade_log::trade_log!(
                    &trader_id,
                    &model.account_id,
                    "",
                    "",
                    "Handled error on set active account.",
                    "err" = &err
                );
            }

            self.app
                .signal_r_message_sender
                .error_publisher
                .send_to_connection(connection, &err.get_message())
                .await;
        }
    }
}
