use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// Partition - Instrument_id
// RowKey - IntervalType as i32
#[my_no_sql_entity("atr-bounds")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtrBoundsEntity {
    pub ignore_height_above: f64,
    pub ignore_height_below: f64,
    pub first_candle_date: String,
    pub last_candle_date_to: String,
    pub candles_count: usize,
    pub heights_count: usize,
    pub upper_height_index: usize,
    pub lower_height_index: usize,
}
