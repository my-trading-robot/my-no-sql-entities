use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//Partition - Instrument_id
//RowKey - g: global, 1m: minute, 1h: hour etc. Consts are in ws_contracts library
#[my_no_sql_entity("price-levels")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceLevelMyNoSqlEntity {}

impl PriceLevelMyNoSqlEntity {
    pub fn new(instrument_id: String, tp: &str, price: f64) -> Self {
        Self {
            partition_key: instrument_id,
            row_key: format!("{}:{}", tp, price),
            time_stamp: Default::default(),
        }
    }
    pub fn get_type(&self) -> &str {
        let index = self.row_key.find(":").unwrap();
        &self.row_key[..index]
    }

    pub fn get_price(&self) -> f64 {
        let index = self.row_key.find(":").unwrap();
        let as_str = &self.row_key[index + 1..];
        as_str.parse().unwrap()
    }
}
