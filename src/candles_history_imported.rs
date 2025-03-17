use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("candles-history-imported")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandlesHistoryImportedMyNoSqlEntity {}

impl CandlesHistoryImportedMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "i"
    }

    pub fn generate_row_key(instr_id: &str) -> &str {
        instr_id
    }
}
