use crate::{CogsApp, views::AppView};
use egui::RichText;

pub struct Login {}

impl AppView for Login {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ectx: &egui::Context) {
        egui::CentralPanel::default().show(ectx, |ui| {
            ui.add_space(10.0);

            ui.vertical(|ui| {
                ui.label(RichText::new("Login").heading());
                ui.add_space(10.0);
            });
        });
    }
}
