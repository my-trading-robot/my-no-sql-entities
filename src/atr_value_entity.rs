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
    pub fn get_instrument(&self) -> &str {
        &self.partition_key
    }

    pub fn get_candle_type(&self) -> &str {
        &self.row_key
    }
}
