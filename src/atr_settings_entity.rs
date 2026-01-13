use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("atr-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtrSettingsEntity {
    pub duration_days: u64,
    pub percent: f64,
    pub candles_count: i32,
    pub candles_limit: Option<usize>,
}

impl AtrSettingsEntity {
    pub const PARTITION_KEY: &'static str = "*";
    pub const ROW_KEY: &'static str = "*";
}

impl Default for AtrSettingsEntity {
    fn default() -> Self {
        Self {
            partition_key: Self::PARTITION_KEY.to_string(),
            row_key: Self::ROW_KEY.to_string(),
            time_stamp: Default::default(),
            duration_days: 365,
            percent: 0.8,
            candles_count: 5,
            candles_limit: None,
        }
    }
}
