use const_format::concatcp;
use egui::{Align, Layout};

use crate::{
    CogsApp,
    comps::{AppComponent, UserWidget},
    consts::{ICON_EXPLORE, ICON_HOME},
    views::{AppView, ViewType},
};

pub struct Header {}

impl AppView for Header {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ectx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .show(ectx, |ui| {
                // The top panel is often a good place for a menu bar:
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    egui::MenuBar::new().ui(ui, |ui| {
                        // Note: There is no File->Quit on web pages.
                        ui.menu_button("File", |ui| {
                            if ui.button("Quit").clicked() {
                                ectx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                        ui.add_space(16.0);
                    });
                }
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(6.0);
                        let logo = egui::include_image!("../../assets/logo_o.png");
                        ui.add(egui::Image::new(logo.clone()).fit_to_original_size(0.04));
                        ui.add_space(10.0);
                        ui.selectable_value(
                            &mut ctx.state.view_type,
                            ViewType::Home,
                            concatcp!(ICON_HOME, "  Home "),
                        );
                        ui.selectable_value(
                            &mut ctx.state.view_type,
                            ViewType::Explore,
                            concatcp!(ICON_EXPLORE, "  Explore "),
                        );
                        egui::global_theme_preference_switch(ui);
                        ui.with_layout(Layout::right_to_left(Align::LEFT), |ui| {
                            ui.add_space(6.0);
                            UserWidget::show(ctx, ui);
                        });
                    });
                });
            });
    }
}
