use std::sync::Arc;

use service_sdk::my_http_server::signal_r::{SignalRMessagePublisher, SignalRPublishersBuilder};

use crate::{
    AccountsSignalRModel, ActivePositionsSignalRModel, BidAsksSignalRModel,
    InstrumentGroupsSignalRModel, InstrumentsSignalRModel, PongSignalRModel,
    PriceChangesSignalRModel, SignalRConnectionContext, SignalRErrorMessage,
    UpdateAccountSignalRModel, UpdateActivePositionSignalRModel, PendingPositionsSignalRModel,
};

pub struct SignalRMessageSender {
    pub accounts_publisher: SignalRMessagePublisher<AccountsSignalRModel, SignalRConnectionContext>,
    pub bidask_publisher: SignalRMessagePublisher<BidAsksSignalRModel, SignalRConnectionContext>,
    pub error_publisher: SignalRMessagePublisher<SignalRErrorMessage, SignalRConnectionContext>,
    pub pong_publisher: SignalRMessagePublisher<PongSignalRModel, SignalRConnectionContext>,
    pub instruments_publisher:
        SignalRMessagePublisher<InstrumentsSignalRModel, SignalRConnectionContext>,
    pub account_update_publisher:
        SignalRMessagePublisher<UpdateAccountSignalRModel, SignalRConnectionContext>,
    pub price_change_publisher:
        SignalRMessagePublisher<PriceChangesSignalRModel, SignalRConnectionContext>,

    pub instruments_groups_publisher:
        SignalRMessagePublisher<InstrumentGroupsSignalRModel, SignalRConnectionContext>,
    pub position_update_publisher:
        SignalRMessagePublisher<UpdateActivePositionSignalRModel, SignalRConnectionContext>,

    pub active_position_publisher:
        SignalRMessagePublisher<ActivePositionsSignalRModel, SignalRConnectionContext>,
    pub pending_position_publisher:
        SignalRMessagePublisher<PendingPositionsSignalRModel, SignalRConnectionContext>,
}

impl SignalRMessageSender {
    pub fn new(builder: &Arc<SignalRPublishersBuilder<SignalRConnectionContext>>) -> Self {
        Self {
            accounts_publisher: builder.get_publisher(),
            error_publisher: builder.get_publisher(),
            bidask_publisher: builder.get_publisher(),
            pong_publisher: builder.get_publisher(),
            instruments_publisher: builder.get_publisher(),
            account_update_publisher: builder.get_publisher(),
            price_change_publisher: builder.get_publisher(),
            instruments_groups_publisher: builder.get_publisher(),
            position_update_publisher: builder.get_publisher(),
            active_position_publisher: builder.get_publisher(),
            pending_position_publisher: builder.get_publisher(),
        }
    }
}
