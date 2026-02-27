use crate::{CogsApp, SourceType, comps::item::ItemWindowState, messages::UiMessage};
use cogs_shared::domain::model::Id;
use egui::{Align, Button, CursorIcon, Layout, Margin, Window, vec2};

pub(super) fn render_ask_window(ctx: &mut CogsApp, ui: &mut egui::Ui, state: &mut ItemWindowState) {
    let ectx = ui.ctx();

    Window::new("new_item_win")
        .title_bar(false)
        .resizable(false)
        .fixed_size(vec2(320.0, 300.0))
        .frame(egui::Frame::window(&ectx.style()).inner_margin(Margin::ZERO))
        .show(ectx, |ui| {
            ui.vertical(|ui| {
                render_header(ui);
                render_content(ctx, ui, state);
                render_footer(ctx, ui, ectx, &state);
                ui.add_space(10.0);
            })
            .response
            .on_hover_cursor(CursorIcon::Grab);
        });
}

fn render_header(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let w = ui.available_width();
        ui.allocate_ui_with_layout(
            egui::vec2(w.max(0.0), 0.0),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.add_enabled(false, egui::Label::new(egui::RichText::new("New Item").size(13.0)));
            },
        );
    });
    ui.add_space(8.0);
}

fn render_content(ctx: &mut CogsApp, ui: &mut egui::Ui, state: &mut ItemWindowState) {
    let mut src_type = match ctx.state.data.new_item_src_type.as_ref() {
        Some(st) => Some(st),
        None => None,
    };
    ui.horizontal(|ui| {
        ui.add_space(14.0);
        ui.label("Create an item from:");
    });
    ui.horizontal(|ui| {
        ui.add_space(18.0);
        ui.vertical(|ui| {
            if ui
                .radio_value(&mut src_type, Some(&SourceType::Scratch), "Scratch")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                log::debug!("Selected from scratch");
                _ = ctx.sendr.send(UiMessage::NewItemFrom(SourceType::Scratch));
            };
            if ui
                .radio_value(&mut src_type, Some(&SourceType::Template), "An item template")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                log::debug!("Selected from template");
                _ = ctx.sendr.send(UiMessage::NewItemFrom(SourceType::Template));
            };
        });
    });
}

fn render_footer(ctx: &mut CogsApp, ui: &mut egui::Ui, ectx: &egui::Context, state: &ItemWindowState) {
    ui.add_space(8.0);
    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        ui.add_space(18.0);

        let enabled = ctx.state.data.new_item_src_type.is_some();
        let resp = ui
            .add_enabled(enabled, Button::new("  Continue  "))
            .on_disabled_hover_text("Provide at least ...");
        if resp.clicked() {
            cleanup(ctx);
        }

        ui.add_space(8.0);

        if ui.button("  Cancel  ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
            cleanup(ctx);
        }
    });
}

fn cleanup(ctx: &mut CogsApp) {
    ctx.state.explore.open_windows_item.remove(&Id::default());
}
