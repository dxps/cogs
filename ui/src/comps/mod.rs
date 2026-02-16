use std::sync::mpsc::Sender;

mod attr_templ_preview;
pub use attr_templ_preview::*;

mod attr_tmpl_win;
pub use attr_tmpl_win::*;

mod dropdown;
pub use dropdown::*;

mod explore_table;
pub use explore_table::*;

mod header;
pub use header::*;

mod footer;
pub use footer::*;

mod item_templ_preview;
pub use item_templ_preview::*;

mod item_tmpl_win;
pub use item_tmpl_win::*;

mod modal;
pub use modal::*;

mod password_input;
pub use password_input::*;

mod user_menu;
pub use user_menu::*;

mod menu_utils;
pub use menu_utils::*;

mod symbols;
pub use symbols::*;

/////////////////////////////////////////////////////////

use crate::messages::UiMessage;
use eframe::egui::Ui;

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn show(ctx: &mut Self::Context, ui: &mut Ui) {}

    #[allow(unused)]
    fn show_input(ui: &mut Ui, value: &mut String) {}

    #[allow(unused)]
    fn show_input_entered(ui: &mut Ui, value: &mut String, entered: &mut bool) {}

    #[allow(unused)]
    fn show_send(ctx: &mut Self::Context, ui: &mut Ui, sendr: Sender<UiMessage>) {}
}
