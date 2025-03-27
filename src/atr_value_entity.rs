use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//Partition - Instrument_id
//RowKey - Interval ("1m", "5m", "1h", "1d", "1M")
#[my_no_sql_entity("atr-values")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtrValueEntity {
    pub value: f64,
}
