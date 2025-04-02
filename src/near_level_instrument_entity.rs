use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("near-level-instruments")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NearLevelInstrumentMyNoSqlEntity {
    pub atr_value: f64,
    pub bid_ask: f64,
}

impl NearLevelInstrumentMyNoSqlEntity {
    pub fn generate_rk(candle_type: &str, level_type: &str, level_price: f64) -> String {
        format!("{candle_type}:{level_type}:{level_price}")
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_candle_type(&self) -> &str {
        let splits = self.row_key.split(":").collect::<Vec<&str>>();

        splits[0]
    }

    pub fn get_level_type(&self) -> &str {
        let splits = self.row_key.split(":").collect::<Vec<&str>>();

        splits[1]
    }

    pub fn get_level_price(&self) -> f64 {
        let splits = self.row_key.split(":").collect::<Vec<&str>>();
        let str = splits[2];

        str.parse().unwrap()
    }
}
