use crate::domain::model::{
    Id,
    meta::{AttributeValueType, NumericAttribute, TextAttribute},
};
// use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// pub trait Attribute {
// fn name(&self) -> String;
// fn value_type(&self) -> AttributeValueType;
// fn text_value(&self) -> String;
// fn smallint_value(&self) -> u16;
// fn int_value(&self) -> u32;
// fn bigint_value(&self) -> u64;
// fn decimal_value(&self) -> f32;
// fn bool_value(&self) -> bool;
// fn date_value(&self) -> NaiveDate;
// fn datetime_value(&self) -> DateTime<chrono::Utc>;
// }
//
// impl Debug for dyn Attribute {
// fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// f.debug_struct("Attribute")
// .field("name", &self.name())
// .field("value_type", &self.value_type())
// TODO: add value, based on value type
// .finish()
// }
// }

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
