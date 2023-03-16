use cfd_engine_sb_contracts::AccountSbModel;

use crate::AccountSignalRModel;

impl From<AccountSbModel> for AccountSignalRModel {
    fn from(src: AccountSbModel) -> Self {
        Self{
            id: src.id,
            balance: src.balance,
            bonus: 0.0,
            currency: src.currency.clone(),
            is_live: true,
            digits: 2,
            symbol: src.currency,
            timestamp: src.last_update_date,
            invest_amount: 0.0,
            achievement_status: "".to_string(),
            free_to_withdrawal: src.balance,
        }
    }
}