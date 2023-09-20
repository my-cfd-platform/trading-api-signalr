use rust_extensions::date_time::DateTimeAsMicroseconds;

pub fn init_signal_r_contract_now() -> i64 {
    return DateTimeAsMicroseconds::now().unix_microseconds / 1000;
}
