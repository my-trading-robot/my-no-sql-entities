use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//Partition - Instrument_id
//RowKey - Interval ("1m", "5m", "1h", "1d", "1M")
#[my_no_sql_entity("trend")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendMyNoSqlEntity {
    pub trend_direction: TrendTypeMyNoSql,
}

impl TrendMyNoSqlEntity {
    pub fn get_instrument(&self) -> &str {
        &self.partition_key
    }

    pub fn get_candle_type(&self) -> &str {
        &self.row_key
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum TrendTypeMyNoSql {
    Up = 0,
    Down = 1,
    Sideways = 2,
}

impl TrendTypeMyNoSql {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Sideways => "Sideways",
        }
    }
}

impl ToString for TrendTypeMyNoSql {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<i32> for TrendTypeMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => TrendTypeMyNoSql::Up,
            1 => TrendTypeMyNoSql::Down,
            2 => TrendTypeMyNoSql::Sideways,
            _ => panic!("Invalid value '{}' for TrendTypeMyNoSql", value,),
        }
    }
}
