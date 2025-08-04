mod attr_tmpl_form;
pub use attr_tmpl_form::*;

mod header;
pub use header::*;

mod footer;
pub use footer::*;

mod modal;
pub use modal::*;

mod password_input;
pub use password_input::*;

mod user_widget;
pub use user_widget::*;

use eframe::egui::Ui;

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn show(ctx: &mut Self::Context, ui: &mut Ui) {}

    #[allow(unused)]
    fn show_input(ui: &mut Ui, value: &mut String) {}
}
