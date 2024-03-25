use std::collections::{BTreeMap, HashMap};

use cfd_engine_sb_contracts::BidAskSbModel;

use crate::BidAskSignalRModel;

pub struct BidAskDirection {
    pub rate: f64,
    pub direction: i32,
}

impl BidAskDirection {
    pub fn new(rate: f64) -> Self {
        Self { rate, direction: 0 }
    }

    pub fn update(&mut self, rate: f64) {
        if rate > self.rate {
            self.direction = 1;
        } else if rate < self.rate {
            self.direction = -1;
        } else {
            self.direction = 0;
        }
    }
}

pub struct BidAskAggregator {
    candles_cache: BTreeMap<u64, HashMap<String, BidAskSignalRModel>>,
    bid_ask_direction: HashMap<String, BidAskDirection>,
}

impl BidAskAggregator {
    pub fn new() -> Self {
        Self {
            candles_cache: BTreeMap::new(),
            bid_ask_direction: HashMap::new(),
        }
    }

    pub fn update(&mut self, bid_ask: &BidAskSbModel) {
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

        let key = bid_ask.date_time_unix / 100000;

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
    }

    pub fn update_vec(&mut self, bid_asks: Vec<BidAskSbModel>) {
        for bid_ask in bid_asks {
            self.update(&bid_ask);
        }
    }

    pub fn get_current_profile(&self) -> Option<&HashMap<String, BidAskSignalRModel>> {
        if let Some(kv) = self.candles_cache.last_key_value() {
            return Some(kv.1);
        }

        return None;
    }
}
