use const_format::concatcp;

use crate::{
    CogsApp,
    icons::{ICON_EXPLORE, ICON_HOME, ICON_SETTINGS},
};

impl CogsApp {
    pub fn header(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                let logo = egui::include_image!("../assets/logo_o.png");
                ui.add(egui::Image::new(logo.clone()));
                ui.add_space(10.0);
                ui.selectable_value(
                    &mut self.view,
                    crate::view::ViewType::Home,
                    concatcp!(ICON_HOME, "  Home "),
                );
                ui.selectable_value(
                    &mut self.view,
                    crate::view::ViewType::Explore,
                    concatcp!(ICON_EXPLORE, "  Explore "),
                );
                ui.selectable_value(
                    &mut self.view,
                    crate::view::ViewType::Settings,
                    concatcp!(ICON_SETTINGS, "  Settings "),
                );
                egui::global_theme_preference_switch(ui);
            });
            ui.add_space(10.0);
        });
    }
}
