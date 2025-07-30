mod header;
pub use header::*;
mod user_widget;
pub use user_widget::*;
mod footer;
pub use footer::*;

use eframe::egui::Ui;

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn show(ctx: &mut Self::Context, ui: &mut Ui) {}
}
