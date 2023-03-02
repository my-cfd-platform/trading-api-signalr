pub struct AccountSignalRModel {
    pub id: String,
    pub balance: f64,
    pub bonus: f64,
    pub currency: String,
    pub is_live: bool,
    pub digits: i32,
    pub symbol: String,
    pub timestamp: u64,
    pub invest_amount: f64,
    pub achievement_status: String,
    pub free_to_withdrawal: i32,
}
