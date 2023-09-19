use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use service_sdk::{
    async_trait,
    my_http_server::signalr::{
        MySignalrActionCallbacks, MySignalrConnection, SignalRTelemetry,
        SignalrContractDeserializer,
    },
};

use crate::{
    handle_signal_r_message, AppContext, SetActiveAccountCommand, SignalRConnectionContext,
    SignalREmptyMessage, SignalRIncomeMessage, SignalRInitAction,
};

//ping
pub struct SignalRPingMessageProcessor {
    app_context: Arc<AppContext>,
}

impl SignalRPingMessageProcessor {
    pub fn new(app_context: Arc<AppContext>) -> Self {
        Self { app_context }
    }

    pub async fn handle_message(
        &self,
        message: SignalRIncomeMessage,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
    ) {
        handle_signal_r_message(&self.app_context, message, connection).await;
    }
}
#[async_trait::async_trait]
impl MySignalrActionCallbacks<SignalREmptyMessage> for SignalRPingMessageProcessor {
    type TCtx = SignalRConnectionContext;

    async fn on(
        &self,
        connection: &Arc<MySignalrConnection<Self::TCtx>>,
        _: Option<HashMap<String, String>>,
        _: SignalREmptyMessage,
        _: &mut SignalRTelemetry,
    ) {
        self.handle_message(SignalRIncomeMessage::Ping, connection)
            .await;
    }
}

//init

pub struct SignalRInitMessageProcessor {
    app_context: Arc<AppContext>,
}

impl SignalRInitMessageProcessor {
    pub fn new(app_context: Arc<AppContext>) -> Self {
        Self { app_context }
    }

    pub async fn handle_message(
        &self,
        message: SignalRIncomeMessage,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
    ) {
        handle_signal_r_message(&self.app_context, message, connection).await;
    }
}

#[async_trait::async_trait]
impl MySignalrActionCallbacks<SignalRInitAction> for SignalRInitMessageProcessor {
    type TCtx = SignalRConnectionContext;

    async fn on(
        &self,
        connection: &Arc<MySignalrConnection<Self::TCtx>>,
        _: Option<HashMap<String, String>>,
        data: SignalRInitAction,
        _: &mut SignalRTelemetry,
    ) {
        self.handle_message(SignalRIncomeMessage::Init(data), connection)
            .await;
    }
}

//set active account

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalRSetActiveAccountMessage {
    pub account_id: String,
}

impl SignalrContractDeserializer for SignalRSetActiveAccountMessage {
    type Item = SignalRSetActiveAccountMessage;

    fn deserialize(data: &[&[u8]]) -> Result<Self::Item, String> {
        return Ok(serde_json::from_slice(data[0]).unwrap());
    }
}

pub struct SignalRSetActiveAccountMessageProcessor {
    app_context: Arc<AppContext>,
}

impl SignalRSetActiveAccountMessageProcessor {
    pub fn new(app_context: Arc<AppContext>) -> Self {
        Self { app_context }
    }

    pub async fn handle_message(
        &self,
        message: SignalRIncomeMessage,
        connection: &Arc<MySignalrConnection<SignalRConnectionContext>>,
    ) {
        handle_signal_r_message(&self.app_context, message, connection).await;
    }
}

#[async_trait::async_trait]
impl MySignalrActionCallbacks<SignalRSetActiveAccountMessage>
    for SignalRSetActiveAccountMessageProcessor
{
    type TCtx = SignalRConnectionContext;

    async fn on(
        &self,
        connection: &Arc<MySignalrConnection<Self::TCtx>>,
        _: Option<HashMap<String, String>>,
        data: SignalRSetActiveAccountMessage,
        _: &mut SignalRTelemetry,
    ) {
        self.handle_message(
            SignalRIncomeMessage::SetActiveAccount(SetActiveAccountCommand::new(data.account_id)),
            connection,
        )
        .await;
    }
}
