use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ELEMENT};
use cogs_shared::domain::model::meta::ItemTemplate;
use egui::{Align, Color32, ComboBox, CursorIcon, Direction, Frame, Grid, Layout, RichText, Window};
use std::sync::{Arc, Mutex};

pub struct ItemTemplateForm {}

impl ItemTemplateForm {
    //
    fn reorder_attrs(element: &mut ItemTemplate, from_idx: usize, to_idx: usize) {
        let attr = element.attributes.remove(from_idx);
        element.attributes.insert(to_idx, attr);
    }
}

impl AppComponent for ItemTemplateForm {
    type Context = CogsApp;

    /// It shows the form for creating or editing an item template.
    /// As params, it expects an `Arc<Mutex<ItemTemplate>>` in `ui.ctx()`'s data key id named `EXPLORE_ELEMENT`.
    fn show(ctx: &mut Self::Context, ui: &mut egui::Ui) {
        //
        let ectx = ui.ctx();
        // Make sure all item templates are loaded.
        if ctx.state.data.get_item_templates().is_empty() {
            ctx.state.data.fetch_all_item_templates(ectx, ctx.sendr.clone());
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
                                ui.label("                  Name");
                                ui.text_edit_singleline(&mut element.name);
                                ui.end_row();

                                ui.label("        Description");
                                ui.text_edit_singleline(&mut element.description);
                                ui.end_row();

                                ui.label("Listing Attribute");
                                ComboBox::from_id_salt(format!("item_templ_form_{}_listing_attr_", id))
                                    .width(285.0)
                                    .selected_text(element.listing_attr.name.clone())
                                    .show_ui(ui, |ui| {
                                        for attr in &element.attributes.clone() {
                                            ui.selectable_value(&mut element.listing_attr, attr.clone(), attr.name.clone());
                                        }
                                    });
                                ui.end_row();

                                ui.label("           Attributes");
                                if !element.attributes.is_empty() {
                                    let mut from_idx = 0;
                                    let mut to_idx = 0;

                                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                        egui::ScrollArea::vertical()
                                            .auto_shrink([true; 2])
                                            .vscroll(false)
                                            .show(ui, |ui| {
                                                //
                                                let frame = Frame::default().corner_radius(3.0).inner_margin(4.0);

                                                ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
                                                    for (idx, item) in &mut element.attributes.iter().enumerate() {
                                                        let item_id = egui::Id::new(item.id.clone());
                                                        let item_idx = idx;

                                                        let response = ui
                                                            .push_id(item_id, |ui| {
                                                                ui.dnd_drag_source(item_id, item_idx, |ui| ui.label(&item.name))
                                                            })
                                                            .response;

                                                        // Detect drops onto this item:
                                                        if let (Some(pointer), Some(hovered_idx)) = (
                                                            ui.input(|i| i.pointer.interact_pos()),
                                                            response.dnd_hover_payload::<usize>(),
                                                        ) {
                                                            // Preview insertion:
                                                            let rect = response.rect;
                                                            let stroke = egui::Stroke::new(1.4, Color32::WHITE);
                                                            let drop_idx: usize = if *hovered_idx == item_idx {
                                                                // We are dragged onto ourselves.
                                                                item_idx
                                                            } else if pointer.y < rect.center().y {
                                                                // Above us.
                                                                ui.painter().hline(
                                                                    rect.x_range().shrink(1.0),
                                                                    rect.top(),
                                                                    stroke,
                                                                );
                                                                item_idx
                                                            } else {
                                                                // Below us.
                                                                ui.painter().hline(
                                                                    rect.x_range().shrink(1.0),
                                                                    rect.bottom(),
                                                                    stroke,
                                                                );
                                                                // item_idx + 1
                                                                item_idx
                                                            };

                                                            let attrs_len = element.attributes.len();
                                                            if let Some(drag_idx) = response.dnd_release_payload::<usize>() {
                                                                log::info!("Dropped {drag_idx} to {drop_idx}.");
                                                                // The user dropped onto this item.
                                                                from_idx = *drag_idx;
                                                                to_idx = match drop_idx {
                                                                    val if val == attrs_len => attrs_len - 1,
                                                                    _ => drop_idx,
                                                                };
                                                            }
                                                        }
                                                    }
                                                    if from_idx != to_idx {
                                                        ItemTemplateForm::reorder_attrs(&mut element, from_idx, to_idx);
                                                    }
                                                });
                                            });
                                    });
                                } else {
                                    ui.label(RichText::new("None").italics().color(Color32::GRAY));
                                }
                                ui.end_row();
                                ui.label("");
                                ui.end_row();

                                ui.label(RichText::new("     Add Attribute").color(Color32::GRAY));
                                let frame = Frame::default().corner_radius(3.0);

                                ui.horizontal(|ui| {
                                    ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
                                        let response = ComboBox::from_id_salt(format!("item_templ_form_{}_add_attr_", id))
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
                                                    if element.attributes.iter().find(|a| a.id == at.id).is_none() {
                                                        ui.selectable_value(
                                                            &mut ctx.state.explore.add_item_template_add_attr_template,
                                                            at.clone(),
                                                            at.name.clone(),
                                                        );
                                                    }
                                                });
                                            })
                                            .response;

                                        // The user dropped an attribute template onto this element.
                                        if let Some(drag_idx) = response.dnd_release_payload::<usize>() {
                                            log::info!(
                                                "[ItemTemplateForm] Attr templ id {drag_idx} is removed from the item template."
                                            );
                                            element.attributes.remove(*drag_idx);
                                        }
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
                })
                .response
                .on_hover_cursor(CursorIcon::Grab);

                ui.add_space(20.0);

                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.add_space(18.0);
                    if ui.button("    Save    ").clicked() {
                        ctx.state
                            .data
                            .save_item_template(element.clone(), ui.ctx(), ctx.sendr.clone());
                        // FYI: This window is closed based on the UiMessage
                        // that is received (by the app itself, in `app.rs`)
                        // after the HTTP call to the svc ends.
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
                                    ctx.state.data.delete_item_template(id.clone(), ectx, ctx.sendr.clone());
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
