use crate::{CogsApp, comps::AppComponent};
use egui::{RichText, Ui, Window};

pub struct AttrTemplateForm {}

impl AppComponent for AttrTemplateForm {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ui: &mut Ui) {
        //
        let title = "New Attribute Template";
        Window::new("AttrTemplateForm")
            .title_bar(false)
            .resizable(false)
            .min_width(400.0)
            .max_width(500.0)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(50.0);
                    ui.label(RichText::new(title).size(13.0));
                    ui.add_space(50.0);
                });
                //
                ui.label("...");
            });
    }
}
