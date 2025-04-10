use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//RowKey - Interval ("1m", "5m", "1h", "1d", "1M")
#[my_no_sql_entity("trend-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendSettingsMyNoSqlEntity {
    pub min_confirmation_ratio: f64,
    pub candles_count: f64,
}

impl TrendSettingsMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "*";

    pub fn get_candle_type(&self) -> &str {
        &self.row_key
    }
}