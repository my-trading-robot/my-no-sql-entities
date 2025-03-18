use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("instruments")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentMyNoSqlEntity {
    pub accuracy: u32,
    pub polygon_instr_id: Option<String>,
}

impl InstrumentMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "i";

    pub fn get_instrument_id(&self) -> &str {
        &self.row_key
    }
}
