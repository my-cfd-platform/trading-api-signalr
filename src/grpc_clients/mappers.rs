use my_nosql_contracts::TradingInstrumentDayOff;

use crate::{accounts_manager::AccountGrpcModel, AccountSignalRModel, InstumentSignalRDayOffModel};

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
