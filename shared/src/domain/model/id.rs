use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Id(pub Uuid);

impl Id {
    pub fn is_zero(&self) -> bool {
        self.0.is_nil()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        match Uuid::parse_str(s) {
            Ok(val) => Self(val),
            Err(e) => {
                log::error!("[Id::from] Failed to parse id. Error: {e}");
                Self::default()
            }
        }
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
