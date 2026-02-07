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

        // Popup style that follows current theme (FRAPPE/LATTE).
        let mut popup_style: egui::Style = ui.style().as_ref().clone();
        let v = ui.visuals();

        // Match popup background to the same family as the trigger (resp) background.
        popup_style.visuals.window_fill = v.widgets.inactive.bg_fill;
        popup_style.visuals.panel_fill = v.widgets.inactive.bg_fill;
        popup_style.visuals.extreme_bg_color = v.widgets.inactive.bg_fill;

        // Optional: keep row hover/active consistent with current theme.
        popup_style.visuals.widgets.inactive.bg_fill = v.widgets.inactive.bg_fill;
        popup_style.visuals.widgets.hovered.bg_fill = v.widgets.hovered.bg_fill;
        popup_style.visuals.widgets.active.bg_fill = v.widgets.active.bg_fill;
        popup_style.visuals.selection.bg_fill = v.selection.bg_fill;

        ui.with_layout(Layout::right_to_left(Align::LEFT), |_ui| {
            Popup::menu(&parent)
                .id(egui::Id::new("user_menu_popup"))
                .style(popup_style)
                .width(POPUP_MIN_WIDTH)
                .gap(5.0)
                .show(|ui| {
                    ui.set_min_width(POPUP_MIN_WIDTH);

                    if ctx.state.auth.user_account.is_none() {
                        if menu_row(ui, concatcp!(ICON_LOGIN, "  Login"), false, None).clicked() {
                            ctx.state.set_curr_view(ViewName::Login);
                        }
                    } else {
                        if menu_row(ui, concatcp!(ICON_SETTINGS, "  Settings"), false, Some(POPUP_MIN_WIDTH)).clicked() {
                            ctx.sendr.send(UiMessage::Settings).unwrap();
                        }

                        ui.add_space(6.0);

                        if menu_row(ui, concatcp!(ICON_LOGOUT, "  Logout"), false, Some(POPUP_MIN_WIDTH)).clicked() {
                            ctx.sendr.send(UiMessage::Logout).unwrap();
                        }
                    }
                });
        });
    }
}
