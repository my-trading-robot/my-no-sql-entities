use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("asset-pair")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentMyNoSqlEntity {
    pub accuracy: u32,
}

impl InstrumentMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "i"
    }

    pub fn generate_row_key(instr_id: &str) -> &str {
        instr_id
    }
}
