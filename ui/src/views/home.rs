use crate::{CogsApp, views::AppView};
use egui::RichText;

pub struct Home {}

impl AppView for Home {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ectx: &egui::Context) {
        egui::CentralPanel::default().show(ectx, |ui| {
            ui.add_space(10.0);
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Home")
                .on_hover_cursor(egui::CursorIcon::Help)
                .on_hover_text("Cogs is a cognitive platform for cognitive needs.");

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Enter label:");
                ui.text_edit_singleline(&mut ctx.label);
            });

            ui.add(egui::Slider::new(&mut ctx.value, 0.0..=10.0).text("value"));

            if ui.button("Increment").clicked() {
                ctx.value += 0.5;
            }

            ui.horizontal(|ui| {
                ui.label(RichText::new("Label:"));
                ui.label(RichText::new(ctx.label.clone()).code());
            });
        });
    }
}
