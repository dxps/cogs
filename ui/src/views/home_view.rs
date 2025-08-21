use crate::{CogsApp, views::AppView};

pub struct Home {}

impl AppView for Home {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ectx: &egui::Context) {
        // The central panel is the region left after adding TopPanel's and SidePanel's.
        egui::CentralPanel::default().show(ectx, |ui| {
            ui.add_space(10.0);
            ui.heading("Home");
            ui.add_space(10.0);

            ui.label("This is a cognitive platform for cognitive needs.");
        });
    }
}
