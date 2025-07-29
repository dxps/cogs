use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewType {
    #[default]
    Home,
    Explore,
    Settings,
    Login,
}
