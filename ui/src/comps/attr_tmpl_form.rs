use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ELEMENT};
use cogs_shared::domain::model::meta::{AttrTemplate, AttributeValueType};
use egui::{Align, Color32, ComboBox, CursorIcon, Direction, Grid, Layout, RichText, Window};
use std::sync::{Arc, Mutex};

pub struct AttrTemplateForm {}

impl AppComponent for AttrTemplateForm {
    type Context = CogsApp;

    /// It shows the form for creating or editing an attribute template.
    /// As params, it expects an `Arc<Mutex<AttrTemplate>>` in `ui.ctx()`'s data key id named `EXPLORE_ELEMENT`.
    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        //
        let ectx = ui.ctx();
        let binding = ectx
            .data(|d| d.get_temp::<Arc<Mutex<AttrTemplate>>>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();
        let mut element = binding.lock().unwrap();

        let id = element.id.clone();
        let title: &str;
        match element.id.is_zero() {
            true => {
                title = "New Attribute Template";
            }
            false => {
                title = "Edit Attribute Template";
            }
        }

        Window::new(format!("AttrTemplateForm_id_{}", element.id))
            .title_bar(false)
            .resizable(false)
            .min_width(300.0)
            .max_width(400.0)
            .min_height(200.0)
            .max_height(400.0)
            .show(ectx, |ui| {
                ui.vertical(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(title).size(13.0));
                        if !id.is_zero() {
                            ui.label(RichText::new(format!("   (id: {})", id)).color(Color32::GRAY).size(10.0));
                        }
                    });
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        Grid::new(format!("attr_templ_id_{}_grid", id))
                            .spacing([10.0, 10.0])
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.label("            Name");
                                // ui.text_edit_singleline(&mut ctx.state.data.curr_attr_template.name);
                                ui.text_edit_singleline(&mut element.name);
                                ui.end_row();
                                ui.label("   Description");
                                ui.text_edit_singleline(&mut element.description);
                                ui.end_row();
                                ui.label("    Value Type");
                                ComboBox::from_id_salt(format!("attr_templ_id_{}_val_type", id))
                                    .width(287.0)
                                    .selected_text(element.value_type.to_string())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut element.value_type,
                                            AttributeValueType::Text,
                                            AttributeValueType::Text.to_string(),
                                        );
                                        ui.selectable_value(
                                            &mut element.value_type,
                                            AttributeValueType::SmallInteger,
                                            AttributeValueType::SmallInteger.to_string(),
                                        );
                                    });
                                ui.end_row();
                                ui.label("Default value");
                                ui.text_edit_singleline(&mut element.default_value);
                                ui.end_row();
                                ui.label("    Mandatory");
                                ui.checkbox(&mut element.is_required, "");
                                ui.end_row();
                            });
                        ui.add_space(8.0);
                    });

                    ui.add_space(20.0);

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.add_space(18.0);
                        if ui.button("    Save    ").clicked() {
                            ctx.state
                                .data
                                .save_attr_template(element.clone(), ui.ctx(), ctx.sendr.clone());
                            ctx.state.explore.open_windows_attr_template.remove(&id);
                        }
                        ui.add_space(8.0);
                        if ui.button("  Cancel  ").clicked() {
                            ctx.state.explore.open_windows_attr_template.remove(&id);
                        }
                        if !element.id.is_zero() {
                            ui.with_layout(
                                Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::Min),
                                |ui| {
                                    ui.add_space(18.0);
                                    if ui.button("  Delete   ").clicked() {
                                        ctx.state.data.delete_attr_template(id.clone(), ectx, ctx.sendr.clone());
                                        ctx.state.explore.open_windows_attr_template.remove(&id);
                                    }
                                },
                            );
                        }
                    });
                    ui.add_space(12.0);
                })
                .response
                .on_hover_cursor(CursorIcon::Grab);
            });
    }
}
