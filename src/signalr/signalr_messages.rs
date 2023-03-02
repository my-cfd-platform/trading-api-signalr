use crate::{SetActiveAccountCommand, SignalRInitAction, InstumentSignalRModel, SignalRMessageWrapperWithAccount, PriceChangeSignalRModel, SignalRMessageWrapper, ActivePositionSignalRModel, AccountSignalRModel, BidAskSignalRModel};

pub enum SignalRIncomeMessage {
    Init(SignalRInitAction),
    SetActiveAccount(SetActiveAccountCommand),
}

pub enum SignalROutcomeMessage {
    Instuments(SignalRMessageWrapperWithAccount<InstumentSignalRModel>),
    PriceChange(SignalRMessageWrapper<PriceChangeSignalRModel>),
    PositionsActive(SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>),
    PendingOrders(SignalRMessageWrapperWithAccount<Vec<ActivePositionSignalRModel>>),
    Accounts(SignalRMessageWrapper<Vec<AccountSignalRModel>>),
    BidAsk(SignalRMessageWrapper<Vec<BidAskSignalRModel>>),
}
