use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//Partition - Instrument_id
//RowKey - Interval ("1m", "5m", "1h", "1d", "1M")
#[my_no_sql_entity("atr-values")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtrValueMyNoSqlEntity {
    pub value: f64,
}

impl AtrValueMyNoSqlEntity {
    pub const MIN_CANDLE_TYPE: &'static str = "1m";
    pub const MIN_5_CANDLE_TYPE: &'static str = "5m";
    pub const HOUR_CANDLE_TYPE: &'static str = "1h";
    pub const DAY_CANDLE_TYPE: &'static str = "1d";
    pub const MONTH_CANDLE_TYPE: &'static str = "1M";

    pub fn get_instrument(&self) -> &str {
        &self.partition_key
    }

    pub fn get_candle_type(&self) -> &str {
        &self.partition_key
    }
}
