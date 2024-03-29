use my_nosql_contracts::TradingInstrumentDayOff;

use crate::{
    accounts_manager_grpc::AccountGrpcModel,
    trading_executor_grpc::{TradingExecutorActivePositionGrpcModel, TradingExecutorPendingPositionGrpcModel}, AccountSignalRModel,
    ActivePositionSignalRModel, InstumentSignalRDayOffModel, SlTpType, PendingPositionSignalRModel,
};

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
        let mut model = ActivePositionSignalRModel {
            id: self.id,
            investment_amount: self.invest_amount,
            open_price: self.open_price,
            open_date: self.open_date,
            instrument: self.asset_pair,
            multiplier: self.leverage,
            operation: self.side.into(),
            swap: self.swaps.iter().map(|x| x.amount).sum(),
            commission: 0.0,
            time_stamp: self.create_date_unix_timestamp_milliseconds / 1000 / 1000,
            tp: None,
            sl: None,
            tp_type: None,
            sl_type: None,
            is_topping_up_active: self.topping_up_percent.is_some(),
            reserved_funds_for_topping_up: self.reserved_funds_for_topping_up,
            base: self.base,
            quote: self.quote,
            collateral: self.collateral,
            base_collateral_open_price: self.base_collateral_open_price,
        };

        if self.sl_in_asset_price.is_some() {
            model.sl = self.sl_in_asset_price;
            model.sl_type = Some(SlTpType::Price);
        };

        if self.sl_in_profit.is_some() {
            model.sl = self.sl_in_profit;
            model.sl_type = Some(SlTpType::Currency);
        };

        if self.tp_in_asset_price.is_some() {
            model.tp = self.tp_in_asset_price;
            model.tp_type = Some(SlTpType::Price);
        };

        if self.tp_in_profit.is_some() {
            model.tp = self.tp_in_profit;
            model.tp_type = Some(SlTpType::Currency);
        };

        model
    }
}



impl Into<PendingPositionSignalRModel> for TradingExecutorPendingPositionGrpcModel {
    fn into(self) -> PendingPositionSignalRModel {
        let mut model = PendingPositionSignalRModel {
            id: self.id,
            investment_amount: self.invest_amount,
            instrument: self.asset_pair,
            multiplier: self.leverage,
            operation: self.side.into(),
            time_stamp: self.create_date_unix_timestamp_milliseconds / 1000 / 1000,
            tp: None,
            sl: None,
            tp_type: None,
            sl_type: None,
            desire_price: self.desire_price,
            is_topping_up_active: self.topping_up_percent.is_some()
        };

        if self.sl_in_asset_price.is_some() {
            model.sl = self.sl_in_asset_price;
            model.sl_type = Some(SlTpType::Price);
        };

        if self.sl_in_profit.is_some() {
            model.sl = self.sl_in_profit;
            model.sl_type = Some(SlTpType::Currency);
        };

        if self.tp_in_asset_price.is_some() {
            model.tp = self.tp_in_asset_price;
            model.tp_type = Some(SlTpType::Price);
        };

        if self.tp_in_profit.is_some() {
            model.tp = self.tp_in_profit;
            model.tp_type = Some(SlTpType::Currency);
        };

        model
    }
}
