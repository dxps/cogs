use crate::domain::model::meta::AttributeValueType;
use chrono::{DateTime, NaiveDate};
use std::fmt::Debug;

pub trait Attribute {
    fn name(&self) -> String;
    fn value_type(&self) -> AttributeValueType;
    fn text_value(&self) -> String;
    fn smallint_value(&self) -> u16;
    fn int_value(&self) -> u32;
    fn bigint_value(&self) -> u64;
    fn decimal_value(&self) -> f32;
    fn bool_value(&self) -> bool;
    fn date_value(&self) -> NaiveDate;
    fn datetime_value(&self) -> DateTime<chrono::Utc>;
}

impl Debug for dyn Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Attribute")
            .field("name", &self.name())
            .field("value_type", &self.value_type())
            // TODO: add value, based on value type
            .finish()
    }
}
