use std::sync::mpsc::Sender;

mod attr_props;
pub use attr_props::*;

mod attr_tmpl_form;
pub use attr_tmpl_form::*;

mod explore_table;
pub use explore_table::*;

mod header;
pub use header::*;

mod footer;
pub use footer::*;

mod link_tmpl_form;
pub use link_tmpl_form::*;

mod modal;
pub use modal::*;

mod password_input;
pub use password_input::*;

mod user_widget;
pub use user_widget::*;

use eframe::egui::Ui;

use crate::messages::UiMessage;

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
