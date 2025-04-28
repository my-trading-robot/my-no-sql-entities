use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//Partition - Instrument_id
//RowKey - Interval ("1h", "1d")
#[my_no_sql_entity("limit-trader")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LimitTraderNoSqlEntity {
    pub level: f64,               
    pub side: LimitTraderSideMyNoSql, 
}

impl LimitTraderNoSqlEntity {
    pub fn get_instrument(&self) -> &str {
        &self.partition_key
    }

    pub fn get_candle_type(&self) -> &str {
        &self.row_key
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum LimitTraderSideMyNoSql {
    Buyer = 0,  // Limit Trader on lows
    Seller = 1, // Limit Trader on highs
}

impl LimitTraderSideMyNoSql {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buyer => "Buyer",
            Self::Seller => "Seller",
        }
    }
}

impl ToString for LimitTraderSideMyNoSql {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<i32> for LimitTraderSideMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => LimitTraderSideMyNoSql::Buyer,
            1 => LimitTraderSideMyNoSql::Buyer,
            _ => panic!("Invalid value '{}' for LimitTraderSideMyNoSql", value,),
        }
    }
}
