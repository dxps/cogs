use crate::domain::model::{
    Id,
    meta::{
        AttributeValueType, BooleanAttribute, DateAttribute, DateTimeAttribute, NumericAttribute,
        TextAttribute,
    },
};
use serde::{Deserialize, Serialize};

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
