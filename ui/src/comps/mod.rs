mod header;
pub use header::*;
mod password_input;
pub use password_input::*;
mod user_widget;
pub use user_widget::*;
mod footer;
pub use footer::*;

use eframe::egui::Ui;

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn show(ctx: &mut Self::Context, ui: &mut Ui) {}

    #[allow(unused)]
    fn show_input(ui: &mut Ui, value: &mut String) {}
}
