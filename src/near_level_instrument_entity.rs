use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("near-level-instruments")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NearLevelInstrumentMyNoSqlEntity {
    pub current_price: f64,
    pub result_price: f64,
}

impl NearLevelInstrumentMyNoSqlEntity {
    pub fn generate_rk(atr: f64, level_price: f64, level_type: &str) -> String {
        format!("{atr}:{level_type}:{level_price}")
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_atr(&self) -> f64 {
        let splits = self.row_key.split(":").collect::<Vec<&str>>();
        let str = splits[0];

        str.parse().unwrap()
    }

    pub fn get_level_type(&self) -> &str {
        let splits = self.row_key.split(":").collect::<Vec<&str>>();

        splits[2]
    }

    pub fn get_level_price(&self) -> f64 {
        let splits = self.row_key.split(":").collect::<Vec<&str>>();
        let str = splits[2];

        str.parse().unwrap()
    }
}
