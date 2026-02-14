use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ELEMENT};
use cogs_shared::domain::model::{
    Action, Id,
    meta::{AttrTemplate, AttributeValueType},
};
use egui::{Align, Button, Color32, ComboBox, CursorIcon, Direction, Grid, Label, Layout, RichText, Window, vec2};
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;

pub struct AttrTemplateWindow;

struct FormUiState {
    id: Id,
    act_id: egui::Id,
    focus_id: egui::Id,
    action: Action,
    title: &'static str,
    focus_name_once: bool,
}

impl FormUiState {
    fn from_ctx(ectx: &egui::Context, element: &AttrTemplate) -> Self {
        let id = element.id.clone();
        let act_id = egui::Id::from(format!("attr_tmpl_id_{}_action", id));
        let action = if id.is_zero() {
            Action::Create
        } else {
            ectx.data(|d| d.get_temp::<Action>(act_id)).unwrap_or(Action::View)
        };

        let title = match action {
            Action::Create => "New Attribute Template",
            Action::Edit => "Edit Attribute Template",
            _ => "View Attribute Template",
        };

        let focus_id = egui::Id::new("new_attr_template_form_focus_name_once");
        let focus_name_once = ectx.data_mut(|d| d.get_temp::<bool>(focus_id).unwrap_or(true));

        Self {
            id,
            act_id,
            focus_id,
            action,
            title,
            focus_name_once,
        }
    }
}

impl AttrTemplateWindow {
    fn render_header(ui: &mut egui::Ui, s: &FormUiState) {
        ui.vertical_centered(|ui| {
            ui.add_enabled(false, Label::new(RichText::new(s.title).size(13.0)));
            if !s.id.is_zero() {
                ui.add_enabled(
                    s.action.is_edit(),
                    Label::new(RichText::new(format!("   (id: {})", s.id)).color(Color32::GRAY).size(10.0)),
                );
            }
        });
    }

    fn render_form_grid(ui: &mut egui::Ui, ectx: &egui::Context, element: &mut AttrTemplate, s: &mut FormUiState) {
        ui.horizontal(|ui| {
            ui.add_space(14.0);

            Grid::new(format!("attr_tmpl_form_{}_grid", s.id))
                .spacing([10.0, 10.0])
                .num_columns(2)
                .show(ui, |ui| {
                    Self::row_name(ui, ectx, element, s);
                    Self::row_description(ui, element, s);
                    Self::row_value_type(ui, element, s);
                    Self::row_default_value(ui, element, s);
                    Self::row_mandatory(ui, element, s);
                });

            ui.add_space(8.0);
        });
    }

    fn render_footer_buttons(
        app: &mut CogsApp,
        ui: &mut egui::Ui,
        ectx: &egui::Context,
        element: &mut AttrTemplate,
        s: &FormUiState,
    ) {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.add_space(18.0);

            if s.action.is_view() {
                if ui.button("    Edit    ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    ectx.data_mut(|d| d.insert_temp(s.act_id, Action::Edit));
                }
            } else {
                let enabled = !element.name.is_empty();
                let resp = ui
                    .add_enabled(enabled, Button::new("    Save    "))
                    .on_disabled_hover_text("Provide the at least a name before saving.");

                if resp.clicked() {
                    app.state
                        .data
                        .save_attr_template(element.clone(), ui.ctx(), app.sendr.clone());
                    cleanup(app, ectx, &s.id, s.act_id, s.focus_id);
                }
            }

            ui.add_space(8.0);

            if ui.button("  Cancel  ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                cleanup(app, ectx, &s.id, s.act_id, s.focus_id);
            }

            if !element.id.is_zero() {
                ui.with_layout(
                    Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::Min),
                    |ui| {
                        ui.add_space(18.0);
                        if ui.button("  Delete   ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                            app.state.data.delete_attr_template(s.id.clone(), ectx, app.sendr.clone());
                            cleanup(app, ectx, &s.id, s.act_id, s.focus_id);
                        }
                    },
                );
            }
        });
    }

    fn row_name(ui: &mut egui::Ui, ectx: &egui::Context, element: &mut AttrTemplate, s: &mut FormUiState) {
        ui.add_enabled(false, Label::new("            Name"));
        let resp = ui.add(egui::TextEdit::singleline(&mut element.name).interactive(!s.action.is_view()));

        if s.action.is_create() && s.focus_name_once {
            resp.request_focus();
            ectx.data_mut(|d| d.insert_temp(s.focus_id, false));
            s.focus_name_once = false;
        }

        ui.end_row();
    }

    fn row_description(ui: &mut egui::Ui, element: &mut AttrTemplate, s: &FormUiState) {
        ui.add_enabled(false, Label::new("   Description"));
        ui.add(egui::TextEdit::singleline(&mut element.description).interactive(!s.action.is_view()));
        ui.end_row();
    }

    fn row_value_type(ui: &mut egui::Ui, element: &mut AttrTemplate, s: &FormUiState) {
        ui.add_enabled(false, Label::new("    Value Type"));
        if s.action.is_view() {
            ui.add(egui::TextEdit::singleline(&mut element.value_type.to_string()).interactive(false));
        } else {
            ComboBox::from_id_salt(format!("atf_val_type_{}", s.id))
                .width(287.0)
                .selected_text(element.value_type.to_string())
                .show_ui(ui, |ui| {
                    for vb in AttributeValueType::iter() {
                        ui.selectable_value(&mut element.value_type, vb.clone(), vb.to_string());
                    }
                });
        }
        ui.end_row();
    }

    fn row_default_value(ui: &mut egui::Ui, element: &mut AttrTemplate, s: &FormUiState) {
        ui.add_enabled(false, Label::new("Default value"));
        ui.add(egui::TextEdit::singleline(&mut element.default_value).interactive(!s.action.is_view()));
        ui.end_row();
    }

    fn row_mandatory(ui: &mut egui::Ui, element: &mut AttrTemplate, s: &FormUiState) {
        ui.add_enabled(false, Label::new("    Mandatory"));
        if s.action.is_view() {
            ui.add_enabled(false, egui::Checkbox::new(&mut element.is_required, ""));
        } else {
            ui.checkbox(&mut element.is_required, "");
        }
        ui.end_row();
    }
}

impl AppComponent for AttrTemplateWindow {
    type Context = CogsApp;

    /// It shows the form for creating or editing an attribute template.
    /// In `ui.ctx().data` it expects an `Arc<Mutex<AttrTemplate>>` under `EXPLORE_ELEMENT`.
    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let ectx = ui.ctx();

        let binding = ectx
            .data(|d| d.get_temp::<Arc<Mutex<AttrTemplate>>>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();

        let mut element = binding.lock().unwrap();
        let mut s = FormUiState::from_ctx(ectx, &element);

        Window::new(format!("attr_tmpl_form_{}_win", element.id))
            .title_bar(false)
            .resizable(false)
            .fixed_size(vec2(400.0, 300.0))
            .show(ectx, |ui| {
                ui.vertical(|ui| {
                    Self::render_header(ui, &s);
                    ui.add_space(20.0);

                    Self::render_form_grid(ui, ectx, &mut element, &mut s);

                    ui.add_space(20.0);
                    Self::render_footer_buttons(ctx, ui, ectx, &mut element, &s);

                    ui.add_space(12.0);
                })
                .response
                .on_hover_cursor(CursorIcon::Grab);
            });
    }
}

fn cleanup(ctx: &mut CogsApp, ectx: &egui::Context, id: &Id, act_id: egui::Id, focus_id: egui::Id) {
    ctx.state.explore.open_windows_attr_template.remove(id);
    ectx.data_mut(|d| d.remove::<Action>(act_id));
    ectx.data_mut(|d| d.remove::<bool>(focus_id));
}
