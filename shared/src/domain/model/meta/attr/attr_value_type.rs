use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// The type of the value of an attribute.
#[derive(Debug, Default, PartialEq, Eq, Clone, Display, EnumIter, Serialize, Deserialize)]
pub enum AttributeValueType {
    //
    /// This is mapped to PostgreSQL's `TEXT` data type.
    #[default]
    #[strum(to_string = "text")]
    Text,

    /// This is mapped to PostgreSQL's `NUMERIC` data type.
    #[strum(to_string = "numeric")]
    Numeric,

    /// This is mapped to PostgreSQL's `boolean` type.
    #[strum(to_string = "boolean")]
    Boolean,

    /// This is mapped to PostgreSQL's `DATE` data type.
    #[strum(to_string = "date")]
    Date,

    /// This is mapped to PostgreSQL's `TIMESTAMP` (without time zone) data type.
    #[strum(to_string = "datetime")]
    DateTime,
}

impl From<&str> for AttributeValueType {
    fn from(value: &str) -> Self {
        // FYI: As before, these string values represent PostgreSQL's types.
        match value {
            "text" => Self::Text,
            "numeric" => Self::Numeric,
            "boolean" => Self::Boolean,
            "date" => Self::Date,
            "datetime" => Self::DateTime,
            _ => Self::Text,
        }
    }
}

impl From<String> for AttributeValueType {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}
