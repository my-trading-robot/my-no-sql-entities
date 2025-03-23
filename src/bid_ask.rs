use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("bid-ask")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAskMyNoSqlEntity {
    pub bid: f64,
    pub ask: f64,
}

impl BidAskMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "s";

    pub fn get_instrument_id(&self) -> &str {
        &self.row_key
    }
}
