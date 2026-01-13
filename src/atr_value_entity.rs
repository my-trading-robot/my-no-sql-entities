use serde::*;
use trading_robot_common::CandleType;

service_sdk::macros::use_my_no_sql_entity!();

//Partition - Instrument_id
//RowKey - Interval ("1m", "5m", "1h", "1d", "1M")
#[my_no_sql_entity("atr-values")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtrValueMyNoSqlEntity {
    pub value: f64,
}

impl AtrValueMyNoSqlEntity {
    pub fn generate_partition_key(instrument_id: &str) -> &str {
        instrument_id
    }

    pub fn generate_row_key(candle_type: CandleType) -> &'static str {
        candle_type.as_str()
    }

    pub fn get_instrument(&self) -> &str {
        &self.partition_key
    }

    pub fn get_candle_type(&self) -> CandleType {
        CandleType::from_str(&self.row_key)
    }
}
