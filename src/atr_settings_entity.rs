use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("atr-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtrSettingsEntity {
    pub duration_days: u64,
    pub percent: f64,
    pub period: i32,
}

impl AtrSettingsEntity {
    pub fn get_rk() -> &'static str {
        "*"
    }

    pub fn get_pk() -> &'static str {
        "*"
    }
}
