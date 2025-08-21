use crate::{CogsApp, views::AppView};

pub struct Settings {}

impl AppView for Settings {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ectx: &egui::Context) {
        egui::CentralPanel::default().show(ectx, |ui| {
            // The central panel is the region left after adding TopPanel's and SidePanel's

            ui.add_space(10.0);
            ui.heading("Settings");
            ui.add_space(10.0);

            ui.label("This is the Settings view.");
        });
    }
}
