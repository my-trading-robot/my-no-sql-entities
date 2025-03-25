use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// Partition - Instrument_id
// RowKey - IntervalType as i32
#[my_no_sql_entity("atr-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtrSettingsEntity {
    pub from_date: i64,
    pub to_date: i64,
    pub percent: f64,
    pub period: i32,
}
