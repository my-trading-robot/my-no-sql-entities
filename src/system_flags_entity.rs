use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//Entity where we keep some statuses. For instance - that we imported candles

// Crypto Candles are imported
//PartitionKey = "cci" - Crypto candle is imported
//RowKey = InstrumentId
//value = NotUsed

#[my_no_sql_entity("system-flags")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemFlagsMyNoSqlEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SystemFlagsMyNoSqlEntity {
    pub const CRYPTO_CANDLE_IS_IMPORTED: &'static str = "cci";
}
