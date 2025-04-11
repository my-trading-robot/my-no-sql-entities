use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//RowKey - Interval ("1m", "5m", "1h", "1d", "1M")
#[my_no_sql_entity("trend-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendSettingsMyNoSqlEntity {
    // 0.0 ... 1.0 Where 1 is 100% 
    pub min_confirmation_ratio: f64,
    pub candles_count: usize,
}

impl TrendSettingsMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "*";

    pub fn get_candle_type(&self) -> &str {
        &self.row_key
    }
}