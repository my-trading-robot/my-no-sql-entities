use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LevelType {
    Global,
    Ath,
    Atl,
}

impl LevelType {
    pub fn from_prefix(prefix: &str) -> Self {
        match prefix {
            PriceLevelMyNoSqlEntity::ROW_KEY_PREFIX_ATH => Self::Ath,
            PriceLevelMyNoSqlEntity::ROW_KEY_PREFIX_ATL => Self::Atl,
            _ => Self::Global,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LevelType::Global => PriceLevelMyNoSqlEntity::ROW_KEY_PREFIX_GLOBAL,
            LevelType::Ath => PriceLevelMyNoSqlEntity::ROW_KEY_PREFIX_ATH,
            LevelType::Atl => PriceLevelMyNoSqlEntity::ROW_KEY_PREFIX_ATL,
        }
    }
}

//Partition - Instrument_id
//RowKey - g: global, 1m: minute, 1h: hour etc. Consts are in ws_contracts library
#[my_no_sql_entity("price-levels")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceLevelMyNoSqlEntity {}

impl PriceLevelMyNoSqlEntity {
    pub const ROW_KEY_PREFIX_GLOBAL: &'static str = "g"; //Global level
    pub const ROW_KEY_PREFIX_ATH: &'static str = "ath"; // ATH (auto updates)
    pub const ROW_KEY_PREFIX_ATL: &'static str = "atl"; // ATL (auto updates)

    pub fn new(instrument_id: String, level_type: LevelType, price: f64) -> Self {
        Self {
            partition_key: instrument_id,
            row_key: format!("{}:{}", level_type.as_str(), price),
            time_stamp: Default::default(),
        }
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.partition_key
    }

    pub fn try_get_type(&self) -> Option<LevelType> {
        let index = self.row_key.find(":")?;
        let prefix = &self.row_key[..index];
        Some(LevelType::from_prefix(prefix))
    }
    pub fn get_type(&self) -> LevelType {
        let index = self.row_key.find(":").unwrap();
        let prefix = &self.row_key[..index];
        LevelType::from_prefix(prefix)
    }

    pub fn get_price(&self) -> f64 {
        let index = self.row_key.find(":").unwrap();
        let as_str = &self.row_key[index + 1..];
        as_str.parse().unwrap()
    }
}
