use crate::{
    CogsApp,
    comps::AppComponent,
    constants::{CORNER_RADIUS, EXPLORE_ELEMENT},
};
use cogs_shared::domain::model::{
    Action, Id,
    meta::{AttrTemplate, ItemTemplate},
};
use egui::{Align, Button, Color32, ComboBox, CursorIcon, Direction, Frame, Grid, Label, Layout, RichText, TextEdit, Window};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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
        // Make sure all attr templates are fetched, as we need them to build an item template.
        if !ctx.state.data.has_fetched_attr_templates() {
            ctx.state.data.fetch_all_attr_templates(ectx, ctx.sendr.clone());
        }

        let binding = ectx
            .data(|d| d.get_temp::<Arc<Mutex<ItemTemplate>>>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();
        let mut element = binding.lock().unwrap();

        let id = element.id.clone();
        let act_id = egui::Id::from(format!("item_tmpl_form_{}_action", id));

        let action = match id.is_zero() {
            true => Action::Create,
            false => ectx.data(|d| d.get_temp::<Action>(act_id)).unwrap_or(Action::View),
        };

        let title = match action {
            Action::Create => "New Item Template",
            Action::Edit => "Edit Item Template",
            _ => "View Item Template",
        };

        let focus_id = egui::Id::new("new_item_template_form_focus_name_once");
        let focus_name_once = ectx.data_mut(|d| d.get_temp::<bool>(focus_id).unwrap_or(true));

        Window::new(format!("item_tmpl_form_{}_win", element.id))
            .title_bar(false)
            .resizable(false)
            .min_width(300.0)
            .max_width(400.0)
            .min_height(200.0)
            .max_height(400.0)
            .show(ectx, |ui| {
                ui.vertical(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_enabled(false, Label::new(RichText::new(title).size(13.0)));
                        if !id.is_zero() {
                            ui.add_enabled(
                                action.is_edit(),
                                Label::new(RichText::new(format!("(id: {})", id)).color(Color32::GRAY).size(10.0)),
                            );
                        }
                    });
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        Grid::new(format!("item_tmpl_id_{}_grid", id))
                            .spacing([10.0, 10.0])
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.add_enabled(false, Label::new("                                   Name"));
                                let resp = ui.add_sized([250.0, ui.spacing().interact_size.y],
                                    TextEdit::singleline(&mut element.name).interactive(!action.is_view()),
                                );
                                if action.is_create() && focus_name_once {
                                    resp.request_focus();
                                    ectx.data_mut(|d| d.insert_temp(focus_id, false));
                                }
                                ui.end_row();
                                ui.add_enabled(false, Label::new("                         Description"));
                                ui.add(TextEdit::singleline(&mut element.description).interactive(!action.is_view()));
                                ui.end_row();

                                ui.add_enabled(false, Label::new("Listing Attribute Template"));
                                if action.is_view() {
                                    ui.add(TextEdit::singleline(&mut element.listing_attr.name).interactive(false));
                                } else {
                                    ComboBox::from_id_salt(format!("item_templ_form_{}_listing_attr_", id))
                                        .width(250.0)
                                        .selected_text(element.listing_attr.name.clone())
                                        .show_ui(ui, |ui| {
                                            for attr in &element.attributes.clone() {
                                                ui.selectable_value(&mut element.listing_attr, attr.clone(), attr.name.clone());
                                            }
                                        });
                                }
                                ui.end_row();

                                ui.add_enabled(false, Label::new("                           Attributes"));
                                if !element.attributes.is_empty() {
                                    let mut from_idx = 0;
                                    let mut to_idx = 0;

                                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                                        egui::ScrollArea::vertical()
                                            .auto_shrink([true; 2])
                                            .vscroll(false)
                                            .show(ui, |ui| {
                                                if action.is_view() {
                                                    let mut attrs_text = element
                                                        .attributes
                                                        .iter()
                                                        .map(|a| a.name.clone())
                                                        .collect::<Vec<_>>()
                                                        .join("\n");
                                                    let rows = element.attributes.len().max(1); // avoid 0 rows
                                                    ui.add(
                                                        TextEdit::multiline(&mut attrs_text)
                                                            .frame(true)
                                                            .interactive(false)
                                                            .desired_rows(rows)
                                                            .desired_width(f32::INFINITY),
                                                    );
                                                } else {
                                                    let frame = Frame::default().corner_radius(CORNER_RADIUS).inner_margin(4.0);
                                                    ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
                                                        ui.set_min_width(242.0);
                                                        for (idx, item) in &mut element.attributes.iter().enumerate() {
                                                            let row_id = egui::Id::new(("item_tmpl_attr_row", element.id.clone(), item.id.clone(), idx));
                                                            let item_idx = idx;

                                                            let response = ui
                                                                .push_id(row_id, |ui| {
                                                                    ui.dnd_drag_source(row_id, item_idx, |ui| {
                                                                        ui.label(&item.name)
                                                                    })
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
                                                }
                                            });
                                    });
                                } else {
                                    ui.label(RichText::new("None").italics().color(Color32::GRAY));
                                }
                                ui.end_row();
                                ui.label("");
                                ui.end_row();

                                if action != Action::View {
                                    ui.add_enabled(false, Label::new("     Add Attribute Template"));

                                    ui.horizontal(|ui| {
                                        let curr_attr_tmpl = ctx.state.explore.item_template_cu_add_attr_template.clone();
                                        let response = ComboBox::from_id_salt(format!("item_templ_form_{}_add_attr_", id))
                                            .width(220.0)
                                            .selected_text(selected_attr_name(&curr_attr_tmpl, &element.id))
                                            .show_ui(ui, |ui| {
                                                let selected_for_element = ctx
                                                    .state
                                                    .explore
                                                    .item_template_cu_add_attr_template
                                                    .entry(element.id.clone())
                                                    .or_insert(None);

                                                for at in ctx.state.data.get_attr_templates() {
                                                    if element.attributes.iter().all(|a| a.id != at.id) {
                                                        ui.selectable_value(selected_for_element, Some(at.clone()), at.name.clone());
                                                    }
                                                }
                                            })
                                            .response;

                                        // The user dropped an attribute template onto this element.
                                        if let Some(drag_idx) = response.dnd_release_payload::<usize>() {
                                            log::info!(
                                                "[ItemTemplateForm] attr tmpl id {drag_idx} is removed from the item template."
                                            );
                                            element.attributes.remove(*drag_idx);
                                            if element.attributes.is_empty() {
                                                element.listing_attr = Default::default();
                                            }
                                        }
                                        let has_selected = ctx.state.explore
                                            .item_template_cu_add_attr_template
                                            .get(&element.id)
                                            .and_then(|o| o.as_ref())
                                            .is_some();

                                        let btn = ui
                                            .add_enabled(has_selected, Button::new(" + "))
                                            .on_disabled_hover_text("Select an attribute template first");

                                        if btn.clicked() {
                                            if let Some(attr) = ctx.state.explore
                                                .item_template_cu_add_attr_template
                                                .get(&element.id)
                                                .and_then(|o| o.clone())
                                            {
                                                element.attributes.push(attr);
                                                ctx.state.explore
                                                    .item_template_cu_add_attr_template
                                                    .insert(element.id.clone(), None);
                                            }
                                        }
                                    });
                                }
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
                    if action.is_view() {
                        if ui.button("    Edit    ")
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked() {
                            ectx.data_mut(|d| d.insert_temp(act_id, Action::Edit));
                        }
                    } else {
                        let enabled = element.name.len() > 0 && !element.attributes.is_empty() && !element.listing_attr.is_default();
                        let resp = ui.add_enabled(enabled, Button::new("    Save    "))
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .on_disabled_hover_text(
                                "Provide the following parts before saving:\n- name\n- listing attribute template\n- one or more attribute templates",
                            );
                        if resp.clicked() {
                            ctx.state
                                .data
                                .save_item_template(element.clone(), ui.ctx(), ctx.sendr.clone());
                            shutdown(ctx, ectx, &id, act_id, focus_id);
                        }
                    }
                    ui.add_space(8.0);
                    if ui.button("  Cancel  ")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked() {
                        shutdown(ctx, ectx, &id, act_id, focus_id);
                    }
                    if !element.id.is_zero() {
                        ui.with_layout(
                            Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::Min),
                            |ui| {
                                ui.add_space(18.0);
                                if ui.button("  Delete  ")
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .clicked() {
                                    ctx.state.data.delete_item_template(id.clone(), ectx, ctx.sendr.clone());
                                    shutdown(ctx, ectx, &id, act_id, focus_id);
                                }
                            },
                        );
                    }
                });
                ui.add_space(12.0);
            });
    }
}

fn selected_attr_name(map: &HashMap<Id, Option<AttrTemplate>>, id: &Id) -> String {
    map.get(id)
        .and_then(|o| o.as_ref())
        .map(|at| at.name.clone())
        .unwrap_or_default()
}

fn shutdown(ctx: &mut CogsApp, ectx: &egui::Context, id: &Id, act_id: egui::Id, focus_id: egui::Id) {
    ctx.state.explore.open_windows_item_template.remove(id);
    ectx.data_mut(|d| d.remove::<Action>(act_id));
    ectx.data_mut(|d| d.remove::<bool>(focus_id));
}
