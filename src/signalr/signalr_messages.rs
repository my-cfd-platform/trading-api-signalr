use crate::{
    AccountSignalRModel, ActivePositionSignalRModel, BidAskSignalRModel, InstumentSignalRModel,
    PriceChangeSignalRModel, SetActiveAccountCommand, SignalRError, SignalRInitAction,
    SignalRMessageWrapper, SignalRMessageWrapperEmpty, SignalRMessageWrapperWithAccount,
};

pub enum SignalRIncomeMessage {
    Init(SignalRInitAction),
    SetActiveAccount(SetActiveAccountCommand),
    Ping,
}

pub enum SignalROutcomeMessage {
    Instruments(SignalRMessageWrapperWithAccount<Vec<InstumentSignalRModel>>),
    PriceChange(SignalRMessageWrapper<PriceChangeSignalRModel>),
    PositionsActive(SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>),
    PendingOrders(SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>),
    Accounts(SignalRMessageWrapper<Vec<AccountSignalRModel>>),
    AccountUpdate(SignalRMessageWrapper<AccountSignalRModel>),
    BidAsk(SignalRMessageWrapper<Vec<BidAskSignalRModel>>),
    Error(SignalRError),
    Pong(SignalRMessageWrapperEmpty),
}
