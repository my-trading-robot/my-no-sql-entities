use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("near-level-instruments")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NearLevelInstrumentMyNoSqlEntity {
    pub price: f64,
    pub atr: f64,
    pub level: f64,
    pub result_price: f64,
}

impl NearLevelInstrumentMyNoSqlEntity {
    pub fn get_instrument_id(&self) -> &str {
        &self.partition_key
    }
    pub fn get_candle_type(&self) -> &str {
        &self.row_key
    }
}
