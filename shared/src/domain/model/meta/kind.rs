#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub enum Kind {
    #[default]
    Item,
    Attribute,
    ItemTemplate,
    AttributeTemplate,
}
