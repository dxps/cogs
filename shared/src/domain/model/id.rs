use std::{fmt::Display, str::FromStr};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Id(pub String);

impl Id {
    pub fn new_from(id: String) -> Self {
        Self(id)
    }

    pub fn new_from_opt(id: &str) -> Option<Self> {
        if id.is_empty() {
            return None;
        }
        Some(Self(id.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl FromStr for Id {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
