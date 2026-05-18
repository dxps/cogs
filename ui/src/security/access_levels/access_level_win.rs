use crate::{comps::AppComponent, constants::EXPLORE_ELEMENT, CogsApp};
use cogs_shared::domain::model::{AccessLevel, Id};
use egui::{vec2, Align, Button, CursorIcon, Layout, Margin, RichText, TextEdit, Window};
use egui_material_icons::{
    icon_button, icon_text,
    icons::{ICON_CLOSE, ICON_DELETE, ICON_EDIT, ICON_INFO},
};

pub struct AccessLevelWindow;

impl AccessLevelWindow {
    fn render_header(ui: &mut egui::Ui, element: &AccessLevel) -> bool {
        let mut should_close = false;

        ui.horizontal(|ui| {
            ui.add_space(18.0);
            ui.label(RichText::new("Access Level").heading());

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(12.0);

                if icon_button(ui, ICON_CLOSE)
                    .on_hover_text("Close")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    should_close = true;
                }

                ui.add_enabled(false, Button::new(icon_text(ICON_EDIT).size(18.0)).frame(false))
                    .on_disabled_hover_text("Access-level editing is not available here yet.");

                ui.add_enabled(false, Button::new(icon_text(ICON_DELETE).size(18.0)).frame(false))
                    .on_disabled_hover_text("Access-level deletion is not available here yet.");

                ui.label(icon_text(ICON_INFO).size(18.0))
                    .on_hover_text(format!("id: {}", element.id))
                    .on_hover_cursor(CursorIcon::Help);
            });
        });

        should_close
    }

    fn render_field(ui: &mut egui::Ui, label: &str, value: &mut String) {
        ui.label(RichText::new(label).strong());
        ui.add(TextEdit::singleline(value).interactive(false).desired_width(f32::INFINITY));
    }
}

impl AppComponent for AccessLevelWindow {
    type Context = CogsApp;

    /// Shows the access-level details window.
    /// In `ui.ctx().data` it expects an `AccessLevel` under `EXPLORE_ELEMENT`.
    fn show(ctx: &mut Self::Context, ui: &mut egui::Ui) {
        let ectx = ui.ctx();
        let element = ectx
            .data(|d| d.get_temp::<AccessLevel>(egui::Id::from(EXPLORE_ELEMENT)))
            .unwrap_or_default();

        let id = element.id.clone();
        let mut name = element.name.clone();
        let mut description = element.description.clone().unwrap_or_default();
        let mut should_close = false;

        Window::new(format!("access_level_{}_win", element.id))
            .title_bar(false)
            .resizable(true)
            .default_size(vec2(650.0, 360.0))
            .min_size(vec2(420.0, 260.0))
            .frame(egui::Frame::window(&ectx.global_style()).inner_margin(Margin::symmetric(24, 20)))
            .show(ectx, |ui| {
                ui.vertical(|ui| {
                    should_close |= Self::render_header(ui, &element);
                    ui.add_space(24.0);

                    ui.add_space(4.0);
                    Self::render_field(ui, "name", &mut name);
                    ui.add_space(22.0);
                    Self::render_field(ui, "description", &mut description);
                })
                .response
                .on_hover_cursor(CursorIcon::Grab);
            });

        if should_close {
            cleanup(ctx, &id);
        }
    }
}

fn cleanup(ctx: &mut CogsApp, id: &Id) {
    ctx.state.explore.open_windows_access_level.remove(id);
}
