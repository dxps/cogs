use crate::{CogsApp, views::AppView};

pub struct SettingsView {}

impl AppView for SettingsView {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // The central panel is the region left after adding TopPanel's and SidePanel's

            ui.add_space(10.0);
            ui.heading("Settings");
            ui.add_space(10.0);

            ui.label("This is the Settings view.");
        });
    }
}
