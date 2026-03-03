use crate::domain::model::{
    Id,
    meta::{AttributeValueType, NumericAttribute, TextAttribute},
};
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
                    Err(format!("Invalid boolean value (of '{value}')").to_string())
                }
            }
            AttributeValueType::Date => todo!(),
            AttributeValueType::DateTime => todo!(),
        }
    }
}
