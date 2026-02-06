use crate::{
    CogsApp,
    comps::AppComponent,
    constants::{ICON_LOGIN, ICON_LOGOUT, ICON_SETTINGS, ICON_USER, POPUP_ROW_H, POPUP_W},
    messages::UiMessage,
    views::ViewName,
};
use const_format::concatcp;
use egui::{Align, Layout, Popup, Sense};

pub struct UserMenu {}

impl AppComponent for UserMenu {
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
                .width(POPUP_W) // set popup width
                .show(|ui| {
                    ui.set_min_width(POPUP_W);

                    fn menu_row(ui: &mut egui::Ui, text: &str, w: f32, h: f32) -> egui::Response {
                        let (rect, response) = ui.allocate_exact_size(egui::vec2(w, h), egui::Sense::click());

                        if response.is_pointer_button_down_on() {
                            ui.painter().rect_filled(rect, 4.0, egui::Color32::from_rgb(70, 70, 90));
                        } else if response.hovered() {
                            ui.painter().rect_filled(rect, 4.0, egui::Color32::from_rgb(55, 55, 70));
                        }

                        let text_pos = egui::pos2(rect.left() + 8.0, rect.center().y);
                        ui.painter().text(
                            text_pos,
                            egui::Align2::LEFT_CENTER,
                            text,
                            egui::TextStyle::Button.resolve(ui.style()),
                            ui.visuals().text_color(),
                        );

                        response.on_hover_cursor(egui::CursorIcon::PointingHand)
                    }

                    if ctx.state.auth.user_account.is_none() {
                        if menu_row(ui, concatcp!(" ", ICON_LOGIN, "  Login "), POPUP_W, POPUP_ROW_H).clicked() {
                            ctx.state.set_curr_view(ViewName::Login);
                        }
                    } else {
                        if menu_row(ui, concatcp!(" ", ICON_SETTINGS, "  Settings "), POPUP_W, POPUP_ROW_H).clicked() {
                            ctx.sendr.send(UiMessage::Settings).unwrap();
                        }

                        ui.add_space(6.0);

                        if menu_row(ui, concatcp!(" ", ICON_LOGOUT, "  Logout "), POPUP_W, POPUP_ROW_H).clicked() {
                            ctx.sendr.send(UiMessage::Logout).unwrap();
                        }
                    }
                });
        });
    }
}
