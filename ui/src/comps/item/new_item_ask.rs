use cogs_shared::domain::model::{Id, meta::ItemTemplate};
use egui::{Align, Button, CursorIcon, Layout, Margin, Window, vec2};

use crate::{CogsApp, comps::AppComponent};

pub struct NewItemWindowAsk;

struct NewItemWindowAskState {
    item_tmpl: Option<ItemTemplate>,
}

impl NewItemWindowAskState {
    fn from_ctx(ectx: &egui::Context) -> Self {
        Self { item_tmpl: None }
    }
}

impl AppComponent for NewItemWindowAsk {
    type Context = CogsApp;

    /// It shows a minimal window to ask the user whether to create an item from an existing
    /// item template or from scratch.
    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let ectx = ui.ctx();
        let mut s = NewItemWindowAskState::from_ctx(ectx);
        Window::new("new_item_win")
            .title_bar(false)
            .resizable(false)
            .fixed_size(vec2(320.0, 300.0))
            .frame(egui::Frame::window(&ectx.style()).inner_margin(Margin::ZERO))
            .show(ectx, |ui| {
                ui.vertical(|ui| {
                    Self::render_header(ui, &s);
                    Self::render_content(ui, &s);
                    Self::render_footer_buttons(ctx, ui, ectx, &s);
                    ui.add_space(10.0);
                })
                .response
                .on_hover_cursor(CursorIcon::Grab);
            });
    }
}

impl NewItemWindowAsk {
    fn render_header(ui: &mut egui::Ui, s: &NewItemWindowAskState) {
        ui.horizontal(|ui| {
            ui.add_space(40.0);
            let w = ui.available_width() - 8.0; // right pad used in grid
            ui.allocate_ui_with_layout(
                egui::vec2(w.max(0.0), 0.0),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.add_enabled(false, egui::Label::new(egui::RichText::new("New Item").size(13.0)));
                },
            );
        });
    }

    fn render_content(ui: &mut egui::Ui, s: &NewItemWindowAskState) {
        ui.horizontal(|ui| {
            ui.add_space(14.0);
            ui.label("Create an item from an existing template or from scratch.");
            ui.add_space(14.0);
        });
    }

    fn render_footer_buttons(app: &mut CogsApp, ui: &mut egui::Ui, ectx: &egui::Context, s: &NewItemWindowAskState) {
        ui.add_space(8.0);
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.add_space(18.0);

            let enabled = s.item_tmpl.is_some();
            let resp = ui
                .add_enabled(enabled, Button::new("  Continue  "))
                .on_disabled_hover_text("Provide at least ...");

            if resp.clicked() {
                cleanup(app, ectx, &Id::default());
            }

            ui.add_space(8.0);

            if ui.button("  Cancel  ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                cleanup(app, ectx, &Id::default());
            }
        });
    }
}

fn cleanup(ctx: &mut CogsApp, ectx: &egui::Context, id: &Id) {
    ctx.state.explore.open_windows_item.remove(id);
}
