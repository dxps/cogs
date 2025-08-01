use crate::{CogsApp, views::AppView};

pub struct Explore {}

impl AppView for Explore {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ectx: &egui::Context) {
        egui::CentralPanel::default().show(ectx, |ui| {
            // The central panel is the region left after adding TopPanel's and SidePanel's

            ui.add_space(10.0);
            ui.heading("Explore");
            ui.add_space(10.0);

            ui.label("This is the explore view.");
        });
    }
}
