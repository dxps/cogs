use crate::{
    CogsApp,
    comps::{AppComponent, menu_row},
    constants::{ICON_LOGIN, ICON_LOGOUT, ICON_SETTINGS, ICON_USER, POPUP_MIN_WIDTH},
    messages::UiMessage,
    views::ViewName,
};
use const_format::concatcp;
use egui::{Align, Layout, Popup, Sense};

pub struct UserMenu {}

impl AppComponent for UserMenu {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let label = match &ctx.state.auth.user_account {
            Some(account) => {
                format!(" {}  {}", ICON_USER, account.name)
            }
            None => format!(" {}", ICON_USER),
        };
        let parent = ui
            .label(label)
            .interact(Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand);

        let mut style = ui.style_mut().clone();
        style.visuals.window_fill = style.visuals.extreme_bg_color;

        ui.with_layout(Layout::right_to_left(Align::LEFT), |_ui| {
            Popup::menu(&parent)
                .id(egui::Id::new("user_menu_popup"))
                .style(style)
                .width(POPUP_MIN_WIDTH)
                .gap(5.0)
                .show(|ui| {
                    ui.set_min_width(POPUP_MIN_WIDTH);

                    if ctx.state.auth.user_account.is_none() {
                        if menu_row(ui, concatcp!(ICON_LOGIN, "  Login"), None).clicked() {
                            ctx.state.set_curr_view(ViewName::Login);
                        }
                    } else {
                        if menu_row(ui, concatcp!(ICON_SETTINGS, "  Settings"), Some(POPUP_MIN_WIDTH)).clicked() {
                            ctx.sendr.send(UiMessage::Settings).unwrap();
                        }

                        ui.add_space(6.0);

                        if menu_row(ui, concatcp!(ICON_LOGOUT, "  Logout"), Some(POPUP_MIN_WIDTH)).clicked() {
                            ctx.sendr.send(UiMessage::Logout).unwrap();
                        }
                    }
                });
        });
    }
}
