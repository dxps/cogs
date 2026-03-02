use crate::{CogsApp, comps::item::ItemWindowState, constants::CORNER_RADIUS};
use cogs_shared::domain::model::{
    Id,
    meta::{AttributeValueType, Item},
};
use egui::{Button, CollapsingHeader, Color32, ComboBox, CursorIcon, Grid, Label, Stroke, TextEdit, Ui};
use strum::IntoEnumIterator;

pub(super) fn render_add_attr(app: &mut CogsApp, ui: &mut Ui, item: &mut Item, state: &mut ItemWindowState) {
    const FORM_FIELD_W: f32 = 200.0;
    let mut name = String::new();
    let mut value_type = None;
    let mut value = String::new();
    if let Some(add_attr) = app.state.explore.item_cu_add_attr.get(&state.id).clone() {
        name = add_attr.name.clone();
        value_type = Some(add_attr.value_type.clone());
        value = add_attr.value.clone();
    } else {
        app.state
            .explore
            .item_cu_add_attr
            .insert(state.id.clone(), Default::default());
    };
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        CollapsingHeader::new("Add an attribute").show(ui, |ui| {
            ui.add_space(10.0);
            Grid::new(format!("item_win_{}_add_attr_grid", state.id))
                .spacing([10.0, 10.0])
                .num_columns(2)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_enabled(false, Label::new(" Name"));
                    });
                    if ui
                        .add_sized([FORM_FIELD_W, ui.spacing().interact_size.y], TextEdit::singleline(&mut name))
                        .changed()
                    {
                        app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap().name = name.clone();
                    }
                    ui.end_row();
                    ui.horizontal(|ui| {
                        ui.add_enabled(false, Label::new("   Type"));
                    });
                    ComboBox::from_id_salt(format!("item_{}_attr_val_type", state.id))
                        .width(FORM_FIELD_W)
                        .selected_text(if let Some(Some(vt)) = &value_type {
                            vt.to_string()
                        } else {
                            String::new()
                        })
                        .show_ui(ui, |ui| {
                            for vb in AttributeValueType::iter() {
                                if ui
                                    .selectable_value(&mut value_type, Some(Some(vb.clone())), vb.to_string())
                                    .changed()
                                {
                                    if let Some(vt) = value_type.clone() {
                                        app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap().value_type = vt;
                                    }
                                };
                            }
                        });
                    ui.end_row();
                    ui.horizontal(|ui| {
                        ui.add_enabled(false, Label::new("  Value"));
                    });
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized([FORM_FIELD_W, ui.spacing().interact_size.y], TextEdit::singleline(&mut value))
                            .changed()
                        {
                            app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap().value = value;
                        }
                        let btn_enabled = if let Some(aa) = app.state.explore.item_cu_add_attr.get(&state.id) {
                            !aa.name.is_empty() && aa.value_type.is_some() && !aa.value.is_empty()
                        } else {
                            false
                        };
                        if ui
                            .add_enabled(btn_enabled, Button::new(" + "))
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .on_disabled_hover_text("Specify name, type, and value\nbefore adding the attribute.")
                            .clicked()
                        {
                            let new_attr = app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap().clone();
                            log::debug!("Adding attr: {:?} to item.", new_attr);
                            item.add_attribute(new_attr);
                            log::debug!("Now, item: {:?}", item);
                            app.state.explore.item_cu_add_attr.remove(&state.id);
                            app.state.explore.open_windows_item.insert(item.id.clone(), item.clone());
                            log::debug!(
                                "[added attr] Updated open_windows_item: {:?}",
                                app.state.explore.open_windows_item
                            );
                        };
                    });
                    ui.end_row();
                });
        });
    });
}

pub(super) fn render_attrs(app: &mut CogsApp, ui: &mut egui::Ui, item: &mut Item, _state: &mut ItemWindowState) {
    let row_h = 20.0;
    let list_h = 260.0;

    // Work on a temporary order that we can reorder via DnD:
    let mut order = item.attributes_order.clone();
    log::debug!("order: {order:?} as per item.attributes_order: {:?}", item.attributes_order);

    let mut changed_value_type = false;
    let mut to_remove: Option<(AttributeValueType, Id)> = None;

    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), list_h),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            egui::ScrollArea::vertical().auto_shrink([true; 2]).show(ui, |ui| {
                egui_dnd::dnd(ui, "rows_dnd").show_vec(
                    &mut order,
                    |ui, ao: &mut (AttributeValueType, Id), handle, _item_state| {
                        ui.push_id(&ao.1, |ui| {
                            let row_w = ui.available_width();

                            let row = ui.allocate_ui_with_layout(
                                egui::vec2(row_w, row_h),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    handle.ui(ui, |ui| {
                                        ui.add_sized([20.0, row_h], egui::Label::new("⠿"));
                                    });

                                    match ao.0 {
                                        AttributeValueType::Text => {
                                            if let Some(attr) = item.text_attributes.clone().iter_mut().find(|a| a.id == ao.1) {
                                                ui.add_sized([120.0, row_h], egui::TextEdit::singleline(&mut attr.name));

                                                let mut value_type = AttributeValueType::Text;
                                                egui::ComboBox::from_id_salt("choice")
                                                    .width(80.0)
                                                    .selected_text(value_type.to_string())
                                                    .show_ui(ui, |ui| {
                                                        for vt in AttributeValueType::iter() {
                                                            if ui
                                                                .selectable_value(&mut value_type, vt.clone(), vt.to_string())
                                                                .changed()
                                                            {
                                                                item.change_attr_value_type(attr.into(), value_type.clone());
                                                                changed_value_type = true;

                                                                app.state
                                                                    .explore
                                                                    .open_windows_item
                                                                    .insert(item.id.clone(), item.clone());

                                                                log::debug!(
                                                                    "[changed text attr] Updated open_windows_item: {:?}",
                                                                    app.state.explore.open_windows_item
                                                                );
                                                            }
                                                        }
                                                    });

                                                ui.add_sized([140.0, row_h], egui::TextEdit::singleline(&mut attr.value));
                                            } else {
                                                ui.label(format!("(missing text) for ao.id={}", ao.1));
                                            }
                                        }
                                        AttributeValueType::Numeric => {
                                            if let Some(attr) = item.numeric_attributes.clone().iter_mut().find(|a| a.id == ao.1)
                                            {
                                                ui.add_sized([120.0, row_h], egui::TextEdit::singleline(&mut attr.name));

                                                let mut value_type = AttributeValueType::Numeric;
                                                egui::ComboBox::from_id_salt("choice")
                                                    .width(80.0)
                                                    .selected_text(value_type.to_string())
                                                    .show_ui(ui, |ui| {
                                                        for vt in AttributeValueType::iter() {
                                                            if ui
                                                                .selectable_value(&mut value_type, vt.clone(), vt.to_string())
                                                                .changed()
                                                            {
                                                                item.change_attr_value_type(attr.into(), value_type.clone());
                                                                changed_value_type = true;

                                                                app.state
                                                                    .explore
                                                                    .open_windows_item
                                                                    .insert(item.id.clone(), item.clone());

                                                                log::debug!(
                                                                    "[changed numeric attr] Updated open_windows_item: {:?}",
                                                                    app.state.explore.open_windows_item
                                                                );
                                                            }
                                                        }
                                                    });

                                                // NOTE: this edits a temporary string; it will not persist.
                                                // Use a temp buffer or egui::DragValue for real numeric editing.
                                                let mut tmp = attr.value.to_string();
                                                if ui.add_sized([140.0, row_h], egui::TextEdit::singleline(&mut tmp)).changed() {
                                                    // parse back if desired:
                                                    // if let Ok(v) = tmp.parse::<f64>() { attr.value = v; }
                                                }
                                            } else {
                                                ui.label(format!("(missing numeric) for a.id={}", ao.1));
                                            }
                                        }
                                        _ => {
                                            ui.label("(not implemented)");
                                        }
                                    }

                                    // Reserve space for the overlay X so we don't cover content.
                                    ui.add_space(row_h);
                                },
                            );

                            let row_rect = row.response.rect;
                            let hovered_row = ui.rect_contains_pointer(row_rect);

                            if hovered_row {
                                let btn_size = egui::vec2(row_h - 5.0, row_h - 5.0);
                                let btn_rect = egui::Rect::from_min_size(
                                    egui::pos2(row_rect.right() - btn_size.x, row_rect.center().y - btn_size.y * 0.6),
                                    btn_size,
                                );

                                // Place the button at a fixed rect within the row (overlay).
                                ui.scope_builder(egui::UiBuilder::new().max_rect(btn_rect), |ui| {
                                    ui.set_clip_rect(row_rect);

                                    if ui
                                        .add(egui::Button::new("x").corner_radius(999.0).fill(egui::Color32::TRANSPARENT))
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .clicked()
                                    {
                                        to_remove = Some((ao.0.clone(), ao.1.clone()));
                                    }
                                });
                            }

                            ui.add_space(2.0);
                        });
                    },
                );
            });
        },
    );

    // Apply removal after list render (avoid mutating while egui_dnd iterates)
    if let Some((vt, id)) = to_remove {
        item.attributes_order.retain(|(_, oid)| *oid != id);
        match vt {
            AttributeValueType::Text => item.text_attributes.retain(|a| a.id != id),
            AttributeValueType::Numeric => item.numeric_attributes.retain(|a| a.id != id),
            AttributeValueType::Boolean => item.boolean_attributes.retain(|a| a.id != id),
            AttributeValueType::Date => item.date_attributes.retain(|a| a.id != id),
            AttributeValueType::DateTime => item.datetime_attributes.retain(|a| a.id != id),
        }

        app.state.explore.open_windows_item.insert(item.id.clone(), item.clone());
        return;
    }

    // Persist reordered list back onto the item, but only if it's a DnD result.
    if order != item.attributes_order && !changed_value_type {
        item.attributes_order = order;
        app.state.explore.open_windows_item.insert(item.id.clone(), item.clone());
        log::debug!(
            "[changed order] Updated open_windows_item: {:?}",
            app.state.explore.open_windows_item
        );
    }
}
