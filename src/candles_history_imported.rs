use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("candles-history-imported")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandlesHistoryImportedMyNoSqlEntity {
    pub max_candle_dt: i64,
}

impl CandlesHistoryImportedMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "i";

    pub fn generate_row_key(instr_id: &str) -> &str {
        instr_id
    }
}
