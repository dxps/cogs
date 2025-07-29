use egui::RichText;

use crate::CogsApp;

impl CogsApp {
    pub fn login(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);

            ui.vertical(|ui| {
                ui.label(RichText::new("Login").heading());
                ui.add_space(10.0);
            });
        });
    }
}
