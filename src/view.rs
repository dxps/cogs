use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ViewType {
    Home,
    Explore,
    Settings,
}
