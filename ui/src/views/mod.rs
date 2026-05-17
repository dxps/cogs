mod home_view;
pub use home_view::*;

mod explore;
pub use explore::*;

mod login_view;
pub use login_view::*;

mod settings_view;
pub use settings_view::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewName {
    #[default]
    Home,
    Explore,
    Settings,
    Login,
}

pub trait AppView {
    type Context;

    #[allow(unused)]
    fn show(ctx: &mut Self::Context, ui: &mut egui::Ui) {}
}
