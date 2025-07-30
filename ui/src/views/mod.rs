mod home;
pub use home::*;

mod login;
pub use login::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewType {
    #[default]
    Home,
    Explore,
    Settings,
    Login,
}

pub trait AppView {
    type Context;

    #[allow(unused)]
    fn show(ctx: &mut Self::Context, ectx: &egui::Context) {}
}
