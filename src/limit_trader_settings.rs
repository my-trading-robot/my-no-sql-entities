use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//RowKey - Interval ("1m", "5m", "1h", "1d", "1M")
#[my_no_sql_entity("limit-trader-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LimitTraderSettingsMyNoSqlEntity {
    pub points_tolerance: u32,
    pub candles_count: usize,
}

impl LimitTraderSettingsMyNoSqlEntity {
    pub fn get_instrument(&self) -> &str {
        &self.partition_key
    }

    pub fn get_candle_type(&self) -> &str {
        &self.row_key
    }
}
    