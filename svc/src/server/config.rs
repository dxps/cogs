#[derive(Debug, Default, serde::Deserialize, PartialEq, Eq)]
pub struct SvcConfig {
    pub listenaddress: String,
}
