use crate::{CogsApp, views::AppView};

pub struct HomeView {}

impl AppView for HomeView {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ui: &mut egui::Ui) {
        // The central panel is the region left after adding TopPanel's and SidePanel's.
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add_space(10.0);
            ui.heading("Home");
            ui.add_space(10.0);

            ui.label("This is a cognitive platform for cognitive needs.");
        });
    }
}
