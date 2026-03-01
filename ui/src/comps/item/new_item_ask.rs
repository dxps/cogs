use crate::{
    CogsApp, SourceType,
    comps::{
        Dropdown, DropdownItem, DropdownStyle,
        item::{ItemWindowState, cleanup},
    },
};
use cogs_shared::domain::model::meta::ItemTemplate;
use egui::{Align, Button, CursorIcon, Layout, Ui};

pub(super) fn render_ask_window(ctx: &mut CogsApp, ui: &mut Ui, state: &mut ItemWindowState<'_>) {
    ui.vertical(|ui| {
        render_header(ui);
        render_ask_body(ctx, ui, state);
        render_ask_footer(ctx, ui, state);
        ui.add_space(10.0);
    })
    .response
    .on_hover_cursor(CursorIcon::Grab);
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

pub(super) fn render_ask_body(ctx: &mut CogsApp, ui: &mut egui::Ui, state: &mut ItemWindowState<'_>) {
    let (mut src_type, src_tmpl) = match state.new_item_src_type_tmpl_cont() {
        Some((sty, ste, _)) => (sty, ste),
        None => (None, None),
    };

    ui.horizontal(|ui| {
        ui.add_space(14.0);
        ui.label("Create an item from:");
    });
    ui.horizontal(|ui| {
        ui.add_space(18.0);
        ui.vertical(|ui| {
            ui.set_min_height(60.0);
            if ui
                .radio_value(&mut src_type, Some(SourceType::Scratch), "scratch")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                state.set_new_item_src_type_tmpl_cont(Some((Some(SourceType::Scratch), None, false)));
            };
            ui.horizontal(|ui| {
                if ui
                    .radio_value(&mut src_type, Some(SourceType::Template), "item template")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    state.set_new_item_src_type_tmpl_cont(Some((Some(SourceType::Template), None, false)));
                };

                if src_type == Some(SourceType::Template) {
                    let templates: Vec<DropdownItem<Option<ItemTemplate>>> = ctx
                        .state
                        .data
                        .get_item_templates()
                        .iter()
                        .map(|it| DropdownItem::new(it.name.clone(), Some(it.clone())))
                        .collect();
                    ui.add_space(4.0);
                    if let Some(v) = Dropdown::show(
                        ui,
                        ui.id().with("new_item_src_tmpl"),
                        &src_tmpl,
                        templates.as_slice(),
                        DropdownStyle {
                            min_width: 174.0,
                            ..Default::default()
                        },
                    ) {
                        state.set_new_item_src_type_tmpl_cont(Some((src_type, v, false)));
                    }
                }
            });
        });
    });
}

pub(super) fn render_ask_footer(ctx: &mut CogsApp, ui: &mut egui::Ui, state: &mut ItemWindowState<'_>) {
    ui.add_space(8.0);
    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        ui.add_space(18.0);

        let (src_type, src_tmpl) = match state.new_item_src_type_tmpl_cont() {
            Some((src_type, src_tmpl, _)) => (src_type, src_tmpl),
            None => (None, None),
        };
        let enabled = src_type == Some(SourceType::Scratch) || src_tmpl.is_some();
        let resp = ui
            .add_enabled(enabled, Button::new("  Continue  "))
            .on_hover_cursor(CursorIcon::PointingHand)
            .on_disabled_hover_text("To continue, select either from scratch\nor choose an existing item template.");
        if resp.clicked() {
            state.set_new_item_src_type_tmpl_cont(Some((src_type, src_tmpl, true))); // set to continue
        }

        ui.add_space(8.0);

        if ui.button("  Cancel  ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
            cleanup(ctx, ui.ctx(), state);
        }
    });
}
