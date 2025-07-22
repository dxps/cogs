use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewType {
    Home,
    Explore,
    Settings,
}
