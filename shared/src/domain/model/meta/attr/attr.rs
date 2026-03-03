use crate::domain::model::{
    Id,
    meta::{
        AttributeValueType, BooleanAttribute, DateAttribute, DateTimeAttribute, NumericAttribute,
        TextAttribute,
    },
};
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Attr {
    pub id: Id,
    pub name: String,
    pub value_type: Option<AttributeValueType>,
    pub value: String,
}

impl From<&mut TextAttribute> for Attr {
    fn from(attr: &mut TextAttribute) -> Self {
        Self {
            id: attr.id.clone(),
            name: attr.name.clone(),
            value_type: Some(AttributeValueType::Text),
            value: attr.value.clone(),
        }
    }
}

impl From<&mut NumericAttribute> for Attr {
    fn from(attr: &mut NumericAttribute) -> Self {
        Self {
            id: attr.id.clone(),
            name: attr.name.clone(),
            value_type: Some(AttributeValueType::Numeric),
            value: attr.value.to_string(),
        }
    }
}

impl From<&mut BooleanAttribute> for Attr {
    fn from(attr: &mut BooleanAttribute) -> Self {
        Self {
            id: attr.id.clone(),
            name: attr.name.clone(),
            value_type: Some(AttributeValueType::Boolean),
            value: attr.value.to_string(),
        }
    }
}

impl From<&mut DateAttribute> for Attr {
    fn from(attr: &mut DateAttribute) -> Self {
        Self {
            id: attr.id.clone(),
            name: attr.name.clone(),
            value_type: Some(AttributeValueType::Date),
            value: attr.value.to_string(),
        }
    }
}

impl From<&mut DateTimeAttribute> for Attr {
    fn from(attr: &mut DateTimeAttribute) -> Self {
        Self {
            id: attr.id.clone(),
            name: attr.name.clone(),
            value_type: Some(AttributeValueType::DateTime),
            value: attr.value.to_string(),
        }
    }
}

impl Attr {
    pub fn validate_value(value_type: &AttributeValueType, value: &str) -> Result<(), String> {
        match value_type {
            AttributeValueType::Text => Ok(()),
            AttributeValueType::Numeric => match Decimal::from_str(value) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            AttributeValueType::Boolean => {
                if value == "true" || value == "false" {
                    Ok(())
                } else {
                    Err(
                        format!("Invalid boolean value (of '{value}').\nIt must be true or false.")
                            .to_string(),
                    )
                }
            }
            AttributeValueType::Date => match value.parse::<NaiveDate>() {
                Ok(_) => Ok(()),
                Err(_) => Err(
                    "The value is not a valid date.\nIt must be in YYYY-MM-DD format".to_string(),
                ),
            },
            AttributeValueType::DateTime => match value.parse::<DateTime<Utc>>() {
                Ok(_) => Ok(()),
                Err(_) => Err(
                    "The value is not a valid datetime.\nIt must be in YYYY-MM-DD hh::mm::ss format."
                        .to_string(),
                ),
            },
        }
    }
}
