use const_format::concatcp;
use egui::{Align, CursorIcon, Layout, Popup, Sense};

use crate::{
    CogsApp,
    comps::AppComponent,
    consts::{ICON_LOGIN, ICON_LOGOUT, ICON_SETTINGS, ICON_USER},
    messages::UiMessage,
    views::ViewType,
};

pub struct UserWidget {}

impl AppComponent for UserWidget {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let mut style = ui.style_mut().clone();
        style.visuals.window_fill = style.visuals.extreme_bg_color;

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

        ui.with_layout(Layout::right_to_left(Align::LEFT), |_ui| {
            Popup::menu(&parent)
                .id(egui::Id::new("user widget popup"))
                .gap(5.0)
                .style(style)
                .show(|ui| {
                    if ctx.state.auth.user_account.is_none() {
                        if ui
                            .label(concatcp!(" ", ICON_LOGIN, "  Login "))
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            ctx.state.view_type = ViewType::Login;
                        };
                    } else {
                        if ui
                            .label(concatcp!(" ", ICON_SETTINGS, "  Settings "))
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            ctx.sendr.send(UiMessage::Settings).unwrap();
                        };
                        ui.add_space(6.0);
                        if ui
                            .label(concatcp!(" ", ICON_LOGOUT, "  Logout "))
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            ctx.sendr.send(UiMessage::Logout).unwrap();
                        }
                    }
                });
        });
    }
}
