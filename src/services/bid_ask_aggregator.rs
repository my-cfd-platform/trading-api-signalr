use std::collections::HashMap;

use cfd_engine_sb_contracts::BidAskSbModel;
use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Timelike, Utc};

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
    candles_cache: HashMap<DateTime<Utc>, HashMap<String, BidAskSignalRModel>>,
    bid_ask_direction: HashMap<String, BidAskDirection>,
}

impl BidAskAggregator {
    pub fn new() -> Self {
        Self {
            candles_cache: HashMap::new(),
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
                self.bid_ask_direction
                    .insert(bid_ask.id.clone(), direction);
                self.bid_ask_direction
                    .get_mut(&bid_ask.id)
                    .unwrap()
            }
        };

        let dt = NaiveDateTime::from_timestamp_millis(bid_ask.date_time_unix_milis as i64).unwrap();
        let year = dt.year();
        let month = dt.month();
        let day = dt.day();
        let hour = dt.hour();
        let min = dt.minute();
        let sec = dt.second();
        let utc = Utc
            .with_ymd_and_hms(year, month, day, hour, min, sec)
            .unwrap();

        if !self.candles_cache.contains_key(&utc) {
            self.candles_cache.insert(utc, HashMap::new());
        }

        let instrument_dict = self.candles_cache.get_mut(&utc).unwrap();

        match instrument_dict.get_mut(&bid_ask.id) {
            Some(candle) => candle.update(bid_ask, direction.direction),
            None => {
                let candle = BidAskSignalRModel::new(bid_ask, direction.direction);
                instrument_dict.insert(bid_ask.id.clone(), candle);
            }
        }
    }

    pub fn get_current_profile(
        &self,
    ) -> &HashMap<DateTime<Utc>, HashMap<String, BidAskSignalRModel>> {
        return &self.candles_cache;
    }
}
