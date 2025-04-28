use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//Partition - Instrument_id
//RowKey - Interval ("1h", "1d")
#[my_no_sql_entity("limit-player")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LimitPlayerNoSqlEntity {
    pub level: f64,               
    pub side: LimitPlayerSideMyNoSql, 
}

impl LimitPlayerNoSqlEntity {
    pub fn get_instrument(&self) -> &str {
        &self.partition_key
    }

    pub fn get_candle_type(&self) -> &str {
        &self.row_key
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum LimitPlayerSideMyNoSql {
    Buyer = 0,  // Limit player on lows
    Seller = 1, // Limit player on highs
}

impl LimitPlayerSideMyNoSql {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buyer => "Buyer",
            Self::Seller => "Seller",
        }
    }
}

impl ToString for LimitPlayerSideMyNoSql {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<i32> for LimitPlayerSideMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => LimitPlayerSideMyNoSql::Buyer,
            1 => LimitPlayerSideMyNoSql::Buyer,
            _ => panic!("Invalid value '{}' for LimitPlayerSideMyNoSql", value,),
        }
    }
}
