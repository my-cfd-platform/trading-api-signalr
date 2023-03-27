use my_nosql_contracts::TradingInstrumentDayOff;

use crate::{accounts_manager::AccountGrpcModel, AccountSignalRModel, InstumentSignalRDayOffModel, ActivePositionSignalRModel, trading_executor::TradingExecutorActivePositionGrpcModel};

impl Into<AccountSignalRModel> for AccountGrpcModel {
    fn into(self) -> AccountSignalRModel {
        AccountSignalRModel {
            id: self.id.clone(),
            balance: self.balance,
            bonus: 0.0,
            currency: self.currency.clone(),
            is_live: true,
            digits: 2,
            symbol: self.currency.clone(),
            timestamp: self.last_update_date,
            invest_amount: 0.0,
            achievement_status: "".to_string(),
            free_to_withdrawal: self.balance,
        }
    }
}

impl Into<InstumentSignalRDayOffModel> for TradingInstrumentDayOff {
    fn into(self) -> InstumentSignalRDayOffModel {
        InstumentSignalRDayOffModel {
            dow_from: self.dow_from,
            time_from: self.time_from,
            dow_to: self.dow_to,
            time_to: self.time_to,
        }
    }
}


impl Into<ActivePositionSignalRModel> for TradingExecutorActivePositionGrpcModel {
    fn into(self) -> ActivePositionSignalRModel {
        ActivePositionSignalRModel{
            id: self.id,
            investment_amount: self.invest_amount,
            open_price: self.open_price,
            open_date: self.open_date,
            instrument: self.asset_pair,
            multiplier: self.leverage,
            operation: self.side.into(),
            swap: 0.0,
            commission: 0.0,
            time_stamp: self.create_date_unix_timestamp_milis / 1000 / 1000,
            tp: None,
            sl: None,
            tp_type: None,
            sl_type: None,
            is_topping_up_active: false,
            reserved_funds_for_topping_up: 0.0,
        }
    }
}