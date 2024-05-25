use service_sdk::rust_extensions::sorted_vec::EntityWithStrKey;

pub struct BidAskDirection {
    pub asset: String,
    pub rate: f64,
    pub direction: i32,
}

impl BidAskDirection {
    pub fn new(asset: String, rate: f64) -> Self {
        Self {
            rate,
            direction: 0,
            asset,
        }
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

impl EntityWithStrKey for BidAskDirection {
    fn get_key(&self) -> &str {
        self.asset.as_str()
    }
}
