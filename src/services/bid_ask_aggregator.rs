use cfd_engine_sb_contracts::BidAskSbModel;
use service_sdk::rust_extensions::sorted_vec::{InsertOrUpdateEntry, SortedVecWithStrKey};

use crate::{BidAskDirection, BidAskSignalRModel};

pub struct BidAskAggregator {
    candles_cache: SortedVecWithStrKey<BidAskSignalRModel>,
    bid_ask_direction: SortedVecWithStrKey<BidAskDirection>,
}

impl BidAskAggregator {
    pub fn new() -> Self {
        Self {
            candles_cache: SortedVecWithStrKey::new(),
            bid_ask_direction: SortedVecWithStrKey::new(),
        }
    }

    pub fn update(&mut self, bid_ask: &BidAskSbModel) {
        let mid = (bid_ask.ask + bid_ask.bid) * 0.5;

        let dir = match self.bid_ask_direction.insert_or_update(&bid_ask.id) {
            InsertOrUpdateEntry::Insert(entry) => {
                let dir = BidAskDirection::new(bid_ask.id.to_string(), mid);
                let result = dir.direction;
                entry.insert(BidAskDirection::new(bid_ask.id.to_string(), mid));
                result
            }
            InsertOrUpdateEntry::Update(entry) => {
                entry.item.update(mid);
                entry.item.direction
            }
        };

        match self.candles_cache.insert_or_update(bid_ask.id.as_str()) {
            InsertOrUpdateEntry::Insert(entry) => {
                entry.insert(BidAskSignalRModel::new(bid_ask, dir))
            }
            InsertOrUpdateEntry::Update(entry) => entry.item.update(bid_ask, dir),
        }

        /*
        let mid = (bid_ask.ask + bid_ask.bid) * 0.5;

        let direction = match self.bid_ask_direction.get_mut(&bid_ask.id) {
            Some(direction) => {
                direction.update(mid);
                direction
            }
            None => {
                let direction = BidAskDirection::new(mid);
                self.bid_ask_direction.insert(bid_ask.id.clone(), direction);
                self.bid_ask_direction.get_mut(&bid_ask.id).unwrap()
            }
        };

        if !self.candles_cache.contains_key(&key) {
            self.candles_cache.insert(key, HashMap::new());
        }

        let instrument_dict = self.candles_cache.get_mut(&key).unwrap();

        match instrument_dict.get_mut(&bid_ask.id) {
            Some(candle) => candle.update(bid_ask, direction.direction),
            None => {
                let candle = BidAskSignalRModel::new(bid_ask, direction.direction);
                instrument_dict.insert(bid_ask.id.clone(), candle);
            }
        }
         */
    }

    pub fn get_current_profile(&mut self) -> Option<Vec<BidAskSignalRModel>> {
        if self.candles_cache.len() == 0 {
            return None;
        }

        let mut result = SortedVecWithStrKey::new();

        std::mem::swap(&mut self.candles_cache, &mut result);

        Some(result.into_vec())
    }
}
