use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
service_sdk::macros::use_my_no_sql_entity!();

/// Partition - InstrumentId
/// RowKey - CandlePatternType
#[my_no_sql_entity("candle-patterns")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandlePatternMyNoSqlEntity {}

impl CandlePatternMyNoSqlEntity {
    pub fn get_instrument_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_pattern_type_str(&self) -> &str {
        &self.row_key
    }
}

#[derive(Debug, Clone)]
pub enum CandlePatternTypeMyNoSqlEntity {
    Retest,
    PressureBuildup,
    AtrSpike,
    Hammer,
    SmallBarApproach,
}

impl TryFrom<&str> for CandlePatternTypeMyNoSqlEntity {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == format!("{:?}", Self::Retest) {
            return Ok(Self::Retest);
        }

        if value == format!("{:?}", Self::PressureBuildup) {
            return Ok(Self::PressureBuildup);
        }

        if value == format!("{:?}", Self::AtrSpike) {
            return Ok(Self::AtrSpike);
        }

        if value == format!("{:?}", Self::Hammer) {
            return Ok(Self::Hammer);
        }

        if value == format!("{:?}", Self::SmallBarApproach) {
            return Ok(Self::SmallBarApproach);
        }

        Err(format!("Unknown CandlePatternTypeMyNoSqlEntity {}", value))
    }
}
