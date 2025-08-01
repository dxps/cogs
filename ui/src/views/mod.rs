mod home;
pub use home::*;

mod explore;
pub use explore::*;

mod login;
pub use login::*;

mod settings;
pub use settings::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
