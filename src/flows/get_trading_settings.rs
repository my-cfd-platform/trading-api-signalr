use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use my_nosql_contracts::{
    BidAskSnapshotNoSqlEntity, PriceChangeSnapshotNoSqlEntity, TradingGroupNoSqlEntity,
    TradingInstrumentNoSqlEntity, TradingProfileNoSqlEntity,
};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

use crate::{
    AppContext, InstrumentGroupSignalRModel, InstrumentSignalRModel, PriceChangeSignalRModel,
    SignalRConnectionContext, SignalRError,
};
/*
pub struct SignalRTradingInfo {
    pub instruments: Vec<Arc<TradingInstrumentNoSqlEntity>>,
    pub profile: Arc<TradingProfileNoSqlEntity>,
    pub instruments_groups: Vec<Arc<TradingInstrumentGroupNoSqlEntity>>,
    pub trading_group: Arc<TradingGroupNoSqlEntity>,
    pub prices: Vec<Arc<BidAskSnapshotNoSqlEntity>>,
    pub price_changes: Vec<PriceChangeSignalRModel>,
}
 */

pub async fn get_trading_entities(
    app: &AppContext,
    ctx: &SignalRConnectionContext,
) -> Result<
    (
        Vec<InstrumentSignalRModel>,
        Vec<InstrumentGroupSignalRModel>,
        Vec<PriceChangeSignalRModel>,
    ),
    SignalRError,
> {
    let account_data = ctx.get_account_data().await?;

    let price_changes = match app
        .price_change_ns_reader
        .get_by_partition_key(PriceChangeSnapshotNoSqlEntity::get_daily_pk())
        .await
    {
        Some(src) => src,
        None => BTreeMap::new(),
    };

    let Some(instruments) = app.instruments_ns_reader.get_table_snapshot_as_vec().await else {
        return Err(SignalRError::InstrumentsNotFound);
    };

    let Some(instrument_groups) = app
        .instruments_groups_ns_reader
        .get_table_snapshot_as_vec()
        .await
    else {
        return Err(SignalRError::InstrumentsGroupsNotFound);
    };

    let Some(_) = app
        .trading_groups_ns_reader
        .get_entity(
            TradingGroupNoSqlEntity::generate_partition_key(),
            &account_data.trading_group_id,
        )
        .await
    else {
        return Err(SignalRError::TradingGroupNotFound);
    };

    let Some(trading_profile) = app
        .trading_profile_ns_reader
        .get_entity(
            TradingProfileNoSqlEntity::generate_partition_key(),
            &account_data.trading_profile_id,
        )
        .await
    else {
        return Err(SignalRError::TradingProfileNotFound);
    };

    let Some(prices_snapshot) = app
        .bid_ask_snapshot_reader
        .get_table_snapshot_as_vec()
        .await
    else {
        return Err(SignalRError::BidAskSnapshotNotFound);
    };

    let (instruments_to_send, price_changes_to_send) = map_instruments(
        instruments,
        &trading_profile,
        price_changes,
        prices_snapshot,
    )
    .await?;

    let instrument_groups_to_send: Vec<InstrumentGroupSignalRModel> = instrument_groups
        .iter()
        .map(|x| x.as_ref().clone().into())
        .collect();

    return Ok((
        instruments_to_send,
        instrument_groups_to_send,
        price_changes_to_send,
    ));
}

async fn map_instruments(
    instruments: Vec<Arc<TradingInstrumentNoSqlEntity>>,
    trading_profile: &TradingProfileNoSqlEntity,
    price_changes: BTreeMap<String, Arc<PriceChangeSnapshotNoSqlEntity>>,
    prices: Vec<Arc<BidAskSnapshotNoSqlEntity>>,
) -> Result<(Vec<InstrumentSignalRModel>, Vec<PriceChangeSignalRModel>), SignalRError> {
    let instruments: HashMap<String, Arc<TradingInstrumentNoSqlEntity>> = instruments
        .into_iter()
        .filter_map(|x| {
            if x.trading_disabled {
                return None;
            }

            return Some((x.get_id().to_string(), x));
        })
        .collect();

    let instruments_to_send: Vec<InstrumentSignalRModel> = trading_profile
        .instruments
        .iter()
        .filter_map(|tp_instrument| {
            let Some(instrument_model) = instruments.get(&tp_instrument.id) else {
                return None;
            };

            if instrument_model.trading_disabled {
                return None;
            }

            let mut instrument = InstrumentSignalRModel {
                id: tp_instrument.id.clone(),
                name: instrument_model.name.clone(),
                digits: instrument_model.digits,
                base: instrument_model.base.clone(),
                quote: instrument_model.quote.clone(),
                day_off: instrument_model
                    .days_off
                    .iter()
                    .map(|day| day.to_owned().into())
                    .collect(),
                min_operation_volume: tp_instrument.min_operation_volume,
                max_operation_volume: tp_instrument.max_operation_volume,
                amount_step_size: 1.0,
                max_position_volume: tp_instrument.max_position_volume,
                stop_out_percent: trading_profile.stop_out_percent,
                multiplier: tp_instrument.leverages.clone(),
                bid: None,
                ask: None,
                group_id: instrument_model.group_id.clone(),
                sub_group_id: None,
                weight: instrument_model.weight,
                markup_bid: None,
                markup_ask: None,
                tick_size: Some(instrument_model.tick_size),
            };

            if let Some(price) = &prices.iter().find(|x| x.row_key == instrument.id) {
                instrument.bid = Some(price.bid);
                instrument.ask = Some(price.ask);
            }

            return Some(instrument);
        })
        .collect();

    return Ok((
        instruments_to_send,
        generate_price_change(price_changes, instruments),
    ));
}

fn generate_price_change(
    price_changes: BTreeMap<String, Arc<PriceChangeSnapshotNoSqlEntity>>,
    instruments: HashMap<String, Arc<TradingInstrumentNoSqlEntity>>,
) -> Vec<PriceChangeSignalRModel> {
    let mut to_send = vec![];

    for (_, price_change) in price_changes {
        if let Some(instrument) = instruments.get(&price_change.row_key) {
            let change = (price_change.current_price - price_change.previous_price)
                / price_change.previous_price
                * 100.0;

            let Some(change) = Decimal::from_f64(change) else {
                continue;
            };

            let change = change.round_dp(instrument.tick_size as u32);

            to_send.push(PriceChangeSignalRModel {
                id: price_change.row_key.clone(),
                chng: change.to_f64().unwrap(),
            });
        }
    }

    return to_send;
}
