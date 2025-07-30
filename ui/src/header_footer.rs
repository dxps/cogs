use crate::{
    CogsApp,
    comps::AppComponent,
    constants::{ICON_EXPLORE, ICON_HOME, ICON_LOGIN, ICON_LOGOUT, ICON_SETTINGS, ICON_USER},
    view::ViewType,
};
use const_format::concatcp;
use egui::{Align, CursorIcon, Layout, Popup, Sense};

impl CogsApp {
    pub fn top_header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    egui::MenuBar::new().ui(ui, |ui| {
                        // Note: There is no File->Quit on web pages.
                        ui.menu_button("File", |ui| {
                            if ui.button("Quit").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                        ui.add_space(16.0);
                    });
                }
                self.header(ui);
            });
    }

    pub fn header(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_space(6.0);
                let logo = egui::include_image!("../assets/logo_o.png");
                ui.add(egui::Image::new(logo.clone()).fit_to_original_size(0.04));
                ui.add_space(10.0);
                ui.selectable_value(
                    &mut self.state.view_type,
                    ViewType::Home,
                    concatcp!(ICON_HOME, "  Home "),
                );
                ui.selectable_value(
                    &mut self.state.view_type,
                    ViewType::Explore,
                    concatcp!(ICON_EXPLORE, "  Explore "),
                );
                egui::global_theme_preference_switch(ui);
                ui.with_layout(Layout::right_to_left(Align::LEFT), |ui| {
                    ui.add_space(6.0);
                    UserWidget::show(self, ui);
                });
            });
        });
    }
}

pub fn footer(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(". ");
        if ui
            .label("Status")
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .clicked()
        {
            let req = ehttp::Request::get("http://localhost:9009/manifest.json");
            ehttp::fetch(req, move |rsp| {
                log::info!("[status] clicked. Test response: {:#?}", rsp);
            });
        }
    });
}

pub struct UserWidget {}

impl AppComponent for UserWidget {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let mut style = ui.style_mut().clone();
        style.visuals.window_fill = style.visuals.extreme_bg_color;
        let label = match ctx.auth_session {
            Some(_) => concatcp!(" ", ICON_USER, "  tbd "),
            None => concatcp!(" ", ICON_USER, "  "),
        };
        let parent = ui
            .label(label)
            .interact(Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        ui.with_layout(Layout::right_to_left(Align::LEFT), |ui| {
            ui.add_space(6.0);
            Popup::menu(&parent)
                .id(egui::Id::new("user widget popup"))
                .gap(5.0)
                .style(style)
                .show(|ui| {
                    if ctx.auth_session.is_none() {
                        if ui
                            .label(concatcp!(ICON_LOGIN, "  Login "))
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            ctx.state.view_type = ViewType::Login;
                        };
                    } else {
                        ui.label(concatcp!(ICON_SETTINGS, "  Settings "))
                            .on_hover_cursor(CursorIcon::PointingHand);
                        ui.label(concatcp!(ICON_LOGOUT, "  Logout "))
                            .on_hover_cursor(CursorIcon::PointingHand);
                    }
                });
        });
    }
}
