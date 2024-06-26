use cfd_engine_sb_contracts::BidAskSbModel;
use serde::{Deserialize, Serialize};
use service_sdk::rust_extensions::{
    date_time::DateTimeAsMicroseconds, sorted_vec::EntityWithStrKey,
};

service_sdk::macros::use_signal_r_json_contract!();

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BidAskSignalRModel {
    pub id: String,
    pub bid: BidAskCandleSignalRModel,
    pub ask: BidAskCandleSignalRModel,
    pub dt: i64,
    pub dir: i32,
}

impl BidAskSignalRModel {
    pub fn update(&mut self, bid_ask: &BidAskSbModel, dir: i32) {
        self.bid.update(bid_ask.bid);
        self.ask.update(bid_ask.ask);
        self.dir = dir;
    }

    pub fn new(bid_ask: &BidAskSbModel, dir: i32) -> BidAskSignalRModel {
        BidAskSignalRModel {
            id: bid_ask.id.clone(),
            bid: BidAskCandleSignalRModel::new(bid_ask.bid),
            ask: BidAskCandleSignalRModel::new(bid_ask.ask),
            dt: DateTimeAsMicroseconds::from(bid_ask.date_time_unix_milis).unix_microseconds / 1000,
            dir,
        }
    }
}

impl EntityWithStrKey for BidAskSignalRModel {
    fn get_key(&self) -> &str {
        self.id.as_str()
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[signal_r_json_contract("bidask")]
pub struct BidAsksSignalRModel {
    pub now: i64,
    pub data: Vec<BidAskSignalRModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAskCandleSignalRModel {
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "c")]
    pub close: f64,
}

impl BidAskCandleSignalRModel {
    pub fn update(&mut self, rate: f64) {
        if rate > self.high {
            self.high = rate;
        }
        if rate < self.low {
            self.low = rate;
        }
        self.close = rate;
    }

    pub fn new(rate: f64) -> Self {
        Self {
            high: rate,
            low: rate,
            open: rate,
            close: rate,
        }
    }

    pub fn apply_markup(&self, markup: f64) -> Self {
        Self {
            high: self.high + markup,
            low: self.low + markup,
            open: self.open + markup,
            close: self.close + markup,
        }
    }
}
