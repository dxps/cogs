use crate::domain::model::{
    Id,
    meta::{
        Attr, AttributeValueType, BooleanAttribute, DateAttribute, DateTimeAttribute,
        NumericAttribute, TextAttribute,
    },
};
use anyhow::Error;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// A template for an item.
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Item {
    //
    pub id: Id,

    /// Its kind, that is its template name, if it was created from a template.
    pub kind: String,

    pub listing_attr_tmpl_id: Id,
    pub listing_attr_name: String,
    pub listing_attr_value: String,

    /// Its template id, if it was created from a template.
    pub tmpl_id: Option<Id>,

    /// The show order of the attributes.
    #[serde(default)]
    pub attributes_order: Vec<(AttributeValueType, Id)>,

    #[serde(default)]
    pub text_attributes: Vec<TextAttribute>,

    #[serde(default)]
    pub numeric_attributes: Vec<NumericAttribute>,

    #[serde(default)]
    pub boolean_attributes: Vec<BooleanAttribute>,

    #[serde(default)]
    pub date_attributes: Vec<DateAttribute>,

    #[serde(default)]
    pub datetime_attributes: Vec<DateTimeAttribute>,
}

impl Item {
    pub fn has_attributes(&self) -> bool {
        self.text_attributes.len() > 0
            || self.numeric_attributes.len() > 0
            || self.boolean_attributes.len() > 0
            || self.date_attributes.len() > 0
            || self.datetime_attributes.len() > 0
    }

    pub fn add_attribute(&mut self, attr: Attr) {
        let attr_id = Id::from(attr.name.clone());
        let id = attr_id.clone();
        let value_type = attr.value_type.clone().unwrap_or_default();
        match value_type {
            AttributeValueType::Text => self.text_attributes.push(TextAttribute {
                id,
                name: attr.name,
                value: attr.value,
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
            AttributeValueType::Numeric => self.numeric_attributes.push(NumericAttribute {
                id,
                name: attr.name,
                value: Decimal::from_str(&attr.value).unwrap_or_default(),
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
            AttributeValueType::Boolean => self.boolean_attributes.push(BooleanAttribute {
                id,
                name: attr.name,
                value: attr.value == "true",
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
            AttributeValueType::Date => self.date_attributes.push(DateAttribute {
                id,
                name: attr.name,
                value: attr.value.parse().unwrap_or_default(),
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
            AttributeValueType::DateTime => self.datetime_attributes.push(DateTimeAttribute {
                id,
                name: attr.name,
                value: attr.value.parse().unwrap_or_default(),
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
        }
        self.attributes_order.push((value_type, attr_id));
    }

    pub fn update_attribute(&mut self, attr: &Attr) -> Result<(), Error> {
        let value_type = attr.value_type.clone().unwrap_or_default();
        match value_type {
            AttributeValueType::Text => {
                if let Some(a) = self.text_attributes.iter_mut().find(|a| a.id == attr.id) {
                    a.name = attr.name.clone();
                    a.value = attr.value.clone();
                }
                Ok(())
            }
            AttributeValueType::Numeric => {
                if let Some(a) = self.numeric_attributes.iter_mut().find(|a| a.id == attr.id) {
                    log::debug!("Updating numeric attr: {a:?} based on {attr:?}");
                    a.name = attr.name.clone();
                    a.value = Decimal::from_str(&attr.value)?;
                }
                Ok(())
            }
            AttributeValueType::Boolean => {
                if let Some(a) = self.boolean_attributes.iter_mut().find(|a| a.id == attr.id) {
                    a.name = attr.name.clone();
                    a.value = attr.value == "true"
                }
                Ok(())
            }
            AttributeValueType::Date => {
                if let Some(a) = self.date_attributes.iter_mut().find(|a| a.id == attr.id) {
                    a.name = attr.name.clone();
                    a.value = attr.value.parse()?;
                }
                Ok(())
            }
            AttributeValueType::DateTime => {
                if let Some(a) = self
                    .datetime_attributes
                    .iter_mut()
                    .find(|a| a.id == attr.id)
                {
                    a.name = attr.name.clone();
                    a.value = attr.value.parse()?;
                }
                Ok(())
            }
        }
    }

    pub fn change_attr_value_type(&mut self, attr: &Attr, to_type: &AttributeValueType) {
        // 1) Remove it from the appropriate vec.
        match attr.value_type.clone().unwrap_or_default() {
            AttributeValueType::Text => self.text_attributes.retain(|a| a.id != attr.id),
            AttributeValueType::Numeric => self.numeric_attributes.retain(|a| a.id != attr.id),
            AttributeValueType::Boolean => self.boolean_attributes.retain(|a| a.id != attr.id),
            AttributeValueType::Date => self.date_attributes.retain(|a| a.id != attr.id),
            AttributeValueType::DateTime => self.datetime_attributes.retain(|a| a.id != attr.id),
        }

        // 2) Add it to the new vec.
        match to_type {
            AttributeValueType::Text => self.text_attributes.push(TextAttribute {
                id: attr.id.clone(),
                name: attr.name.clone(),
                value: attr.value.clone(),
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
            AttributeValueType::Numeric => self.numeric_attributes.push(NumericAttribute {
                id: attr.id.clone(),
                name: attr.name.clone(),
                value: match Decimal::from_str(&attr.value) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!(
                            "Failed to parse value '{}' for numeric attribute '{}': {}. Defaulting to 0.",
                            &attr.value,
                            attr.name,
                            e
                        );
                        Decimal::new(0, 0) // default to 0 if parsing fails
                    }
                },
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
            AttributeValueType::Boolean => self.boolean_attributes.push(BooleanAttribute {
                id: attr.id.clone(),
                name: attr.name.clone(),
                value: attr.value == "true",
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
            AttributeValueType::Date => self.date_attributes.push(DateAttribute {
                id: attr.id.clone(),
                name: attr.name.clone(),
                value: attr.value.parse().unwrap_or_default(),
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
            AttributeValueType::DateTime => self.datetime_attributes.push(DateTimeAttribute {
                id: attr.id.clone(),
                name: attr.name.clone(),
                value: attr.value.parse().unwrap_or_default(),
                tmpl_id: None,
                owner_id: self.id.clone(),
            }),
        }

        // 3) Reflect it in the attributes_order as well.
        self.attributes_order.iter_mut().for_each(|(vt, id)| {
            if *id == attr.id {
                *vt = to_type.clone();
            }
        });

        log::debug!(
            "Changed value_type={} for attr named '{}'. Updated item: {:?}",
            to_type,
            &attr.name,
            self
        );
    }
}
