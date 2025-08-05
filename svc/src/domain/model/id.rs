use std::{fmt::Display, str::FromStr};

use snowflake_ng::{SnowflakeGenerator, provider::STD_PROVIDER};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Id(pub i64);

impl Id {
    pub fn new() -> Self {
        let id_gen = SnowflakeGenerator::default();
        let val = id_gen.assign_sync(&STD_PROVIDER);
        Id { 0: *val }
    }

    pub fn new_from_opt(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }
        let val = i64::from_str(s).ok()?;
        Some(Self(val))
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        let val = i64::from_str(s);
        if val.is_err() {
            log::error!("Invalid provided id: {}", s);
        }
        Self(val.unwrap())
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl FromStr for Id {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<i64> for Id {
    fn from(value: i64) -> Self {
        Self { 0: value }
    }
}
