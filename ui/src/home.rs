use egui::RichText;

use crate::{CogsApp, header_footer::footer};

impl CogsApp {
    pub fn home(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Home")
                .on_hover_cursor(egui::CursorIcon::Help)
                .on_hover_text("Cogs is a cognitive platform for cognitive needs.");

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Enter label:");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));

            if ui.button("Increment").clicked() {
                self.value += 0.5;
            }

            ui.horizontal(|ui| {
                ui.label(RichText::new("Label:"));
                ui.label(RichText::new(self.label.clone()).code());
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                footer(ui);
                // egui::warn_if_debug_build(ui);
            });
        });
    }
}
