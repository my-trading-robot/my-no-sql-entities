use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

const MINUTE_KEY: &'static str = "1m";
const MINUTE_5_KEY: &'static str = "5m";
const HOUR_KEY: &'static str = "1h";
const DAY_KEY: &'static str = "1d";
const MONTH_KEY: &'static str = "1M";

#[derive(Clone, Copy)]
pub enum MyNoSqlCandleType {
    Minute,
    Minute5,
    Hour,
    Day,
    Month,
}
impl MyNoSqlCandleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Minute => MINUTE_KEY,
            Self::Minute5 => MINUTE_5_KEY,
            Self::Hour => HOUR_KEY,
            Self::Day => DAY_KEY,
            Self::Month => MONTH_KEY,
        }
    }

    pub fn from_str(src: &str) -> Self {
        match src {
            MINUTE_KEY => Self::Minute,
            MINUTE_5_KEY => Self::Minute5,
            HOUR_KEY => Self::Hour,
            DAY_KEY => Self::Day,
            MONTH_KEY => Self::Month,
            _ => {
                panic!("Invalid candle interval key: {}", src);
            }
        }
    }
}

//Partition - Instrument_id
//RowKey - Interval ("1m", "5m", "1h", "1d", "1M")
#[my_no_sql_entity("atr-values")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtrValueMyNoSqlEntity {
    pub value: f64,
}

impl AtrValueMyNoSqlEntity {
    pub fn generate_partition_key(instrument_id: &str) -> &str {
        instrument_id
    }

    pub fn generate_row_key(candle_type: MyNoSqlCandleType) -> &'static str {
        candle_type.as_str()
    }

    pub fn get_instrument(&self) -> &str {
        &self.partition_key
    }

    pub fn get_candle_type(&self) -> MyNoSqlCandleType {
        MyNoSqlCandleType::from_str(&self.row_key)
    }
}
