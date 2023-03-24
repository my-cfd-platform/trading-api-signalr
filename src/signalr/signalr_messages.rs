use crate::{
    AccountSignalRModel, ActivePositionSignalRModel, BidAskSignalRModel, InstrumentSignalRModel,
    PriceChangeSignalRModel, SetActiveAccountCommand, SignalRError, SignalRInitAction,
    SignalRMessageWrapper, SignalRMessageWrapperEmpty, SignalRMessageWrapperWithAccount, InstrumentGroupSignalRModel,
};

pub enum SignalRIncomeMessage {
    Init(SignalRInitAction),
    SetActiveAccount(SetActiveAccountCommand),
    Ping,
}

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
    Error(SignalRError),
    Pong(SignalRMessageWrapperEmpty),
}
