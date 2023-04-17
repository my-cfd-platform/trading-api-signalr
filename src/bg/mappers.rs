use cfd_engine_sb_contracts::{AccountSbModel, OrderSbModel, PositionPersistenceEvent};

use crate::{AccountSignalRModel, ActivePositionSignalRModel, ActivePositionSignalRSideModel, SlTpType};

pub enum SbPositionPersistenceUpdateType {
    Create(OrderSbModel),
    Update(OrderSbModel),
    Close(OrderSbModel),
}

impl SbPositionPersistenceUpdateType {
    pub fn extract_trader_id(&self) -> &str{
        match self {
            SbPositionPersistenceUpdateType::Create(order) => &order.trader_id,
            SbPositionPersistenceUpdateType::Update(order) => &order.trader_id,
            SbPositionPersistenceUpdateType::Close(order) => &order.trader_id,
        }
    }
}

impl From<PositionPersistenceEvent> for SbPositionPersistenceUpdateType {
    fn from(value: PositionPersistenceEvent) -> Self {
        if let Some(order) = value.create_position {
            return SbPositionPersistenceUpdateType::Create(order);
        }

        if let Some(order) = value.update_position {
            return SbPositionPersistenceUpdateType::Update(order);
        }

        if let Some(order) = value.close_position {
            return SbPositionPersistenceUpdateType::Close(order);
        }

        panic!("Unknown position persistence event type: {:?}", value);
    }
}

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
        let mut model = Self {
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
        };

        if src.sl_in_instrument_price.is_some() {
            model.sl = src.sl_in_instrument_price;
            model.sl_type = Some(SlTpType::Price);
        };

        if src.sl_in_currency.is_some() {
            model.sl = src.sl_in_currency;
            model.sl_type = Some(SlTpType::Currency);
        };

        if src.tp_in_instrument_price.is_some() {
            model.tp = src.tp_in_instrument_price;
            model.tp_type = Some(SlTpType::Price);
        };

        if src.tp_in_currency.is_some() {
            model.tp = src.tp_in_currency;
            model.tp_type = Some(SlTpType::Currency);
        };

        model
    }
}
