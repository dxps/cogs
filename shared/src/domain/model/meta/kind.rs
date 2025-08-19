#[derive(Clone, Default, PartialEq, Eq, Hash, Debug, serde::Deserialize, serde::Serialize)]
pub enum Kind {
    #[default]
    Item,
    Attribute,
    ItemTemplate,
    AttributeTemplate,
}
