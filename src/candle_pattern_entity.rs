use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
use std::fmt::Display;
service_sdk::macros::use_my_no_sql_entity!();

/// Partition - InstrumentId
/// RowKey - CandlePatternType
#[my_no_sql_entity("candle-patterns")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandlePatternMyNoSqlEntity {
    pub direction: String,
}

impl CandlePatternMyNoSqlEntity {
    pub fn generate_rk(candle_type: &str, pattern_type: CandlePatternTypeMyNoSqlEntity) -> String {
        format!("{candle_type}:{}", pattern_type.to_string())
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_pattern_type_str(&self) -> &str {
        let splits = self.row_key.split(":").collect::<Vec<&str>>();

        splits[1]
    }

    pub fn get_candle_type_str(&self) -> &str {
        let splits = self.row_key.split(":").collect::<Vec<&str>>();

        splits[0]
    }
}

#[derive(Debug, Clone)]
pub enum CandlePatternTypeMyNoSqlEntity {
    CloseRetest,
    LongRetest,
    PressureBuildup,
    AtrSpike,
    Hammer,
    SmallBarApproach,
}

impl Display for CandlePatternTypeMyNoSqlEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CandlePatternTypeMyNoSqlEntity::CloseRetest => write!(f, "CloseRetest"),
            CandlePatternTypeMyNoSqlEntity::LongRetest => write!(f, "LongRetest"),
            CandlePatternTypeMyNoSqlEntity::PressureBuildup => write!(f, "PressureBuildup"),
            CandlePatternTypeMyNoSqlEntity::AtrSpike => write!(f, "AtrSpike"),
            CandlePatternTypeMyNoSqlEntity::Hammer => write!(f, "Hammer"),
            CandlePatternTypeMyNoSqlEntity::SmallBarApproach => write!(f, "SmallBarApproach"),
        }
    }
}

impl TryFrom<&str> for CandlePatternTypeMyNoSqlEntity {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == Self::CloseRetest.to_string() {
            return Ok(Self::CloseRetest);
        }

        if value == Self::LongRetest.to_string() {
            return Ok(Self::LongRetest);
        }

        if value == Self::PressureBuildup.to_string() {
            return Ok(Self::PressureBuildup);
        }

        if value == Self::AtrSpike.to_string() {
            return Ok(Self::AtrSpike);
        }

        if value == Self::Hammer.to_string() {
            return Ok(Self::Hammer);
        }

        if value == Self::SmallBarApproach.to_string() {
            return Ok(Self::SmallBarApproach);
        }

        Err(format!("Unknown CandlePatternTypeMyNoSqlEntity {}", value))
    }
}
