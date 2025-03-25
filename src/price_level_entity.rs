use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//Partition - Instrument_id
//RowKey - g: global, 1m: minute, 1h: hour etc. Consts are in ws_contracts library
#[my_no_sql_entity("price-levels")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceLevelEntity {
    pub price: f64,
}
