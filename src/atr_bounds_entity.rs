use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// Partition - Instrument_id
// RowKey - IntervalType as i32
#[my_no_sql_entity("atr-bounds")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtrBoundsEntity {
    pub ignore_height_above: f64,
    pub ignore_height_below: f64,
}
