use cfd_engine_sb_contracts::{AccountSbModel, OrderSbModel};

use crate::{AccountSignalRModel, ActivePositionSignalRModel, ActivePositionSignalRSideModel};

impl From<AccountSbModel> for AccountSignalRModel {
    fn from(src: AccountSbModel) -> Self {
        Self {
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

impl From<OrderSbModel> for ActivePositionSignalRModel {
    fn from(src: OrderSbModel) -> Self {
        Self {
            id: src.id,
            investment_amount: src.invest_amount,
            open_price: src.open_price,
            open_date: src.open_date,
            instrument: src.asset_pair,
            multiplier: src.leverage,
            operation: ActivePositionSignalRSideModel::from(src.side),
            swap: 0.0,
            commission: 0.0,
            time_stamp: src.create_date,
            tp: None,
            sl: None,
            tp_type: None,
            sl_type: None,
            is_topping_up_active: false,
            reserved_funds_for_topping_up: 0.0,
        }
    }
}
