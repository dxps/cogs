use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ELEMENT};
use cogs_shared::domain::model::meta::{AttrTemplate, ItemTemplate};
use egui::{Align, Color32, ComboBox, Direction, Frame, Grid, Layout, RichText, Window};
use std::sync::{Arc, Mutex};

pub struct ItemTemplateForm {}

impl AppComponent for ItemTemplateForm {
    type Context = CogsApp;

    /// It shows the form for creating or editing an item template.
    /// As params, it expects an `Arc<Mutex<ItemTemplate>>` in `ui.ctx()`'s data key id named `EXPLORE_ELEMENT`.
    fn show(ctx: &mut Self::Context, ui: &mut egui::Ui) {
        //
        let ectx = ui.ctx();

        // Make sure all attribute templates are loaded.
        if ctx.state.data.get_attr_templates().is_empty() {
            ctx.state.data.fetch_all_attr_templates(ectx, ctx.sendr.clone());
        }

        let binding = ectx
            .data(|d| d.get_temp::<Arc<Mutex<ItemTemplate>>>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();
        let mut element = binding.lock().unwrap();

        let id = element.id.clone();
        let title: &str;
        match element.id.is_zero() {
            true => {
                title = "New Item Template";
            }
            false => {
                title = "Edit Item Template";
            }
        }

        Window::new(format!("ItemTemplateForm_id_{}", element.id))
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
                            ui.label(
                                RichText::new(format!("   (id: {})", id))
                                    .color(Color32::GRAY)
                                    .size(10.0),
                            );
                        }
                    });
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        Grid::new(format!("attr_templ_id_{}_grid", id))
                            .spacing([10.0, 10.0])
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.label("                  Name");
                                ui.text_edit_singleline(&mut element.name);
                                ui.end_row();
                                ui.label("        Description");
                                ui.text_edit_singleline(&mut element.description);
                                ui.end_row();
                                ui.label("Listing Attribute");
                                ComboBox::from_id_salt(format!("item_templ_form_{}_listing_attr_", id))
                                    .width(285.0)
                                    .selected_text(
                                        ctx.state.explore.add_item_template_listing_attr_template.name.clone(),
                                    )
                                    .show_ui(ui, |ui| {
                                        for attr in &element.attributes {
                                            ui.selectable_value(
                                                &mut ctx.state.explore.add_item_template_listing_attr_template,
                                                attr.clone(),
                                                attr.name.clone(),
                                            );
                                        }
                                    });
                                ui.end_row();
                                ui.label("           Attributes");
                                if !element.attributes.is_empty() {
                                    let frame = Frame::default().inner_margin(2.0);
                                    ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
                                        for (idx, attr) in element.attributes.iter().enumerate() {
                                            let id = egui::Id::new(("item_templ_form_attr_dnd_", idx));
                                            let _resp = ui.dnd_drag_source(id, idx, |ui| {
                                                //
                                            });
                                        }
                                    });
                                } else {
                                    ui.label(RichText::new("None").italics().color(Color32::GRAY));
                                }
                                ui.end_row();
                                ui.label("");
                                ui.end_row();

                                ui.label(RichText::new("     Add Attribute").color(Color32::GRAY));
                                ui.horizontal(|ui| {
                                    ComboBox::from_id_salt(format!("item_templ_form_{}_add_attr_", id))
                                        .width(256.0)
                                        .selected_text(
                                            if ctx.state.explore.add_item_template_add_attr_template.id.is_zero() {
                                                "".to_string()
                                            } else {
                                                ctx.state.explore.add_item_template_add_attr_template.name.clone()
                                            },
                                        )
                                        .show_ui(ui, |ui| {
                                            ctx.state.data.get_attr_templates().iter().for_each(|at| {
                                                ui.selectable_value(
                                                    &mut ctx.state.explore.add_item_template_add_attr_template,
                                                    at.clone(),
                                                    at.name.clone(),
                                                );
                                            });
                                        });
                                    if ui.button(" + ").clicked() {
                                        let elem = ctx.state.explore.add_item_template_add_attr_template.clone();
                                        element.attributes.push(elem);
                                        ctx.state.explore.add_item_template_add_attr_template = Default::default();
                                        log::debug!("[ItemTemplateForm] Its attributes: {:#?}", element.attributes);
                                    }
                                });
                                ui.end_row();
                            });
                        ui.add_space(8.0);
                    });
                });

                ui.add_space(20.0);

                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.add_space(18.0);
                    if ui.button("    Save    ").clicked() {
                        // ctx.state
                        // .data
                        // .save_attr_template(element.clone(), ui.ctx(), ctx.sendr.clone());
                        ctx.state.explore.open_windows_item_template.remove(&id);
                    }
                    ui.add_space(8.0);
                    if ui.button("  Cancel  ").clicked() {
                        ctx.state.explore.open_windows_item_template.remove(&id);
                    }
                    if !element.id.is_zero() {
                        ui.with_layout(
                            Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::Min),
                            |ui| {
                                ui.add_space(18.0);
                                if ui.button("  Delete   ").clicked() {
                                    // ctx.state.data.delete_attr_template(id.clone(), ectx, ctx.sendr.clone());
                                    ctx.state.explore.open_windows_item_template.remove(&id);
                                }
                            },
                        );
                    }
                });
                ui.add_space(12.0);
            });
    }
}
