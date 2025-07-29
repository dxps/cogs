use std::sync::{Arc, RwLock};

use cogs_shared::domain::model::UserAccount;
use const_format::concatcp;
use egui::{Align, CursorIcon, Layout, Popup, Response, Sense};

use crate::{
    CogsApp,
    app::AppState,
    constants::{ICON_EXPLORE, ICON_HOME, ICON_LOGIN, ICON_LOGOUT, ICON_SETTINGS, ICON_USER},
    view::ViewType,
};

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
                ui.add_space(10.0);
                let logo = egui::include_image!("../assets/logo_o.png");
                ui.add(egui::Image::new(logo.clone()));
                ui.add_space(10.0);
                ui.selectable_value(
                    &mut self.state.write().unwrap().view_type,
                    ViewType::Home,
                    concatcp!(ICON_HOME, "  Home "),
                );
                ui.selectable_value(
                    &mut self.state.write().unwrap().view_type,
                    ViewType::Explore,
                    concatcp!(ICON_EXPLORE, "  Explore "),
                );
                egui::global_theme_preference_switch(ui);
                ui.with_layout(Layout::right_to_left(Align::LEFT), |ui| {
                    let label = match self.auth_session {
                        Some(_) => concatcp!(" ", ICON_USER, "  Login "),
                        None => concatcp!(" ", ICON_USER, "   "),
                    };
                    ui.add_space(6.0);
                    let parent = ui
                        .label(label)
                        .interact(Sense::click())
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    self.user_widget.parent_widget = Some(parent);
                    if self.user_widget.parent_widget.as_ref().unwrap().clicked() {
                        self.user_widget.open_popup = true;
                    };
                    self.user_widget.show_popup(ui);
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

pub struct UserWidget {
    pub parent_widget: Option<Response>,
    pub auth_session: Option<UserAccount>,
    pub open_popup: bool,
    pub state: Arc<RwLock<AppState>>,
}

impl UserWidget {
    pub fn new(state: Arc<RwLock<AppState>>) -> Self {
        Self {
            parent_widget: None,
            auth_session: None,
            open_popup: false,
            state,
        }
    }
    fn show_popup(&mut self, ui: &mut egui::Ui) {
        let mut style = ui.style_mut().clone();
        style.visuals.window_fill = style.visuals.extreme_bg_color;
        Popup::menu(self.parent_widget.as_ref().unwrap())
            .id(egui::Id::new("user widget popup"))
            .gap(5.0)
            .style(style)
            .show(|ui| {
                if self.auth_session.is_none() {
                    if ui
                        .label(concatcp!(ICON_LOGIN, "  Login "))
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    {
                        log::info!("[show_popup] Login clicked. Setting view_type to Login");
                        self.state.write().unwrap().view_type = ViewType::Login;
                        log::info!(
                            "[show_popup] After that, view_type: {:#?}",
                            self.state.read().unwrap().view_type
                        );
                    };
                } else {
                    ui.label(concatcp!(ICON_SETTINGS, "  Settings "))
                        .on_hover_cursor(CursorIcon::PointingHand);
                    ui.label(concatcp!(" ", ICON_LOGOUT, "  Logout "))
                        .on_hover_cursor(CursorIcon::PointingHand);
                }
            });
    }
}
