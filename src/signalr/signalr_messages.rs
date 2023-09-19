use crate::{
    AccountSignalRModel, ActivePositionSignalRModel, BidAskSignalRModel, InstrumentSignalRModel,
    PriceChangeSignalRModel, SetActiveAccountCommand, SignalRInitAction,
    SignalRMessageWrapper, SignalRMessageWrapperEmpty, SignalRMessageWrapperWithAccount, InstrumentGroupSignalRModel, SignalRErrorMessage,
};

pub enum SignalRIncomeMessage {
    Init(SignalRInitAction),
    SetActiveAccount(SetActiveAccountCommand),
    Ping,
}

#[derive(Debug, Clone)]
pub enum SignalROutcomeMessage {
    Instruments(SignalRMessageWrapperWithAccount<Vec<InstrumentSignalRModel>>),
    InstrumentsGroups(SignalRMessageWrapperWithAccount<Vec<InstrumentGroupSignalRModel>>),
    PriceChange(SignalRMessageWrapper<Vec<PriceChangeSignalRModel>>),
    PositionsActive(SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>),
    PendingOrders(SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>),
    Accounts(SignalRMessageWrapper<Vec<AccountSignalRModel>>),
    AccountUpdate(SignalRMessageWrapper<AccountSignalRModel>),
    BidAsk(SignalRMessageWrapper<Vec<BidAskSignalRModel>>),
    PositionUpdate(SignalRMessageWrapperWithAccount<ActivePositionSignalRModel>),
    ActivePositions(SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>),
    Error(SignalRErrorMessage),
    Pong(SignalRMessageWrapperEmpty),
}
