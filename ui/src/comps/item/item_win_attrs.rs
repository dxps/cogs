use crate::{
    CogsApp,
    colors::{faded_color, faded_red_color},
    comps::item::ItemWindowState,
    constants::{ICON_REORDER, ICON_X_DEL},
};
use cogs_shared::domain::model::{
    Id,
    meta::{Attr, AttributeValueType, DateAttribute, DateTimeAttribute, Item},
};
use egui::{Button, CollapsingHeader, Color32, ComboBox, CursorIcon, Grid, Label, RichText, Stroke, TextEdit, Ui};
use std::collections::HashMap;
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
    let mut attr_name_already_included = item.has_attribute(&name);
    let mut valid_value = None;

    ui.horizontal(|ui| {
        ui.add_space(10.0);
        CollapsingHeader::new(RichText::new("Add an attribute").color(faded_color())).show(ui, |ui| {
            ui.add_space(10.0);
            Grid::new(format!("item_win_{}_add_attr_grid", state.id))
                .spacing([10.0, 10.0])
                .num_columns(2)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_enabled(false, Label::new(" Name"));
                    });
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized([FORM_FIELD_W, ui.spacing().interact_size.y], TextEdit::singleline(&mut name))
                            .changed()
                        {
                            app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap().name = name.clone();
                            attr_name_already_included = item.has_attribute(&name);
                        };
                        if attr_name_already_included {
                            ui.label(RichText::new("(!)").color(Color32::RED))
                                .on_hover_text_at_pointer(RichText::new(
                                    "You cannot add an attribute with\nthe same name as an existing one.",
                                ));
                        }
                    });

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
                                        let attr = app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap();
                                        if vt == Some(AttributeValueType::Date) {
                                            attr.value = DateAttribute::now_value().to_string();
                                        } else if vt == Some(AttributeValueType::DateTime) {
                                            attr.value = DateTimeAttribute::now_value().to_string();
                                            log::debug!("[render_add_attr | value_type changed] Generated and put '{}' datetime value to the attr.", attr.value);
                                        }
                                        attr.value_type = vt;
                                    }
                                };
                            }
                        });
                    ui.end_row();
                    ui.horizontal(|ui| {
                        ui.add_enabled(false, Label::new("  Value"));
                    });
                    ui.horizontal(|ui| {
                        let attr = app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap();
                        if attr.value_type == Some(AttributeValueType::Boolean) {
                            let mut bval = value == "true";
                            if ui.checkbox(&mut bval, "").changed() {
                                attr.value = bval.to_string();
                            }
                            let desired = egui::vec2(174.0, 22.0);
                            let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
                            ui.put(rect, egui::Label::new(""));
                        } else {
                            let mut value_input = TextEdit::singleline(&mut value);
                            if attr.value_type == Some(AttributeValueType::Numeric) {
                                if let Err(e) = Attr::validate_value(&AttributeValueType::Numeric, &attr.value){
                                    valid_value = Some(e);
                                    value_input = value_input.background_color(faded_red_color());
                                } else {
                                    valid_value = None;
                                };
                            }
                            let mut resp = ui.add_sized([FORM_FIELD_W, ui.spacing().interact_size.y], value_input);
                            if let Some(ref e) = valid_value {
                                resp = resp.on_hover_text_at_pointer(e.clone());
                            }
                            if resp.changed()
                            {
                                attr.value = value;
                            }
                        }
                        
                        let btn_enabled = !attr.name.is_empty() && attr.value_type.is_some() && !attr.value.is_empty() 
                            && !attr_name_already_included && valid_value.is_none();
                        let hover_label = if attr_name_already_included {
                            "Cannot add an attribute with\nthe same name as an existing one.".to_string()
                        } else if let Some(e) = valid_value {
                            e.clone()
                        } else {
                            "Specify name, type, and value\nbefore adding the attribute.".to_string()
                        };
                        if ui
                            .add_enabled(btn_enabled, Button::new(" + "))
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .on_disabled_hover_text(hover_label)
                            .clicked()
                        {
                            // let new_attr = app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap().clone();
                            // log::debug!("Adding attr: {:?} to item.", new_attr);
                            // item.add_attribute(new_attr);
                            log::debug!("Adding attr: {:?} to item.", attr);
                            item.add_attribute(attr.clone());
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

    // Work on a temporary order that we can reorder via DnD.
    let mut order = item.attributes_order.clone();

    let mut changed_value_type = false;
    let mut to_remove: Option<(AttributeValueType, Id)> = None;
    // If an attribute has invalid value (entered in is corresponding input value),
    // it will be added to this map and have the error included.
    let mut invalid_values = HashMap::<String, String>::new();

    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), list_h),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.add_sized(
                    [26.0, row_h],
                    Label::new(egui::RichText::new("type").size(11.0).color(faded_color())),
                );
                ui.add_sized(
                    [136.0, row_h],
                    Label::new(egui::RichText::new("name").size(11.0).color(faded_color())),
                );
                ui.add_sized(
                    [144.0, row_h],
                    Label::new(egui::RichText::new("value").size(11.0).color(faded_color())),
                );
            });
            egui::ScrollArea::vertical().auto_shrink([true; 2]).show(ui, |ui| {
                egui_dnd::dnd(ui, "rows_dnd").show_vec(
                    &mut order,
                    |ui, ao: &mut (AttributeValueType, Id), handle, _item_state| {
                        ui.push_id(&ao.1, |ui| {
                            let row_w = ui.available_width();

                            ui.horizontal(|ui| {
                                let row = ui.allocate_ui_with_layout(
                                    egui::vec2(row_w, row_h),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.add_space(18.0);
                                        match ao.0 {
                                            ////////// Text //////////
                                            AttributeValueType::Text => {
                                                if let Some(attr) = item.text_attributes.clone().iter_mut().find(|a| a.id == ao.1)
                                                {
                                                    let mut value_type = AttributeValueType::Text;
                                                    egui::ComboBox::from_id_salt("choice")
                                                        .width(74.0)
                                                        .selected_text(
                                                            RichText::new(value_type.to_string()).color(faded_color()).size(12.0),
                                                        )
                                                        .show_ui(ui, |ui| {
                                                            for vt in AttributeValueType::iter() {
                                                                if ui
                                                                    .selectable_value(&mut value_type, vt.clone(), vt.to_string())
                                                                    .changed()
                                                                {
                                                                    update(
                                                                        item,
                                                                        app,
                                                                        attr.into(),
                                                                        Some(value_type.clone()),
                                                                        None,
                                                                    );
                                                                    changed_value_type = true;
                                                                }
                                                            }
                                                        });

                                                    if ui
                                                        .add_sized([140.0, row_h], TextEdit::singleline(&mut attr.name))
                                                        .changed()
                                                    {
                                                        update(item, app, attr.into(), None, None);
                                                    };
                                                    if ui
                                                        .add_sized([160.0, row_h], TextEdit::singleline(&mut attr.value))
                                                        .changed()
                                                    {
                                                        update(item, app, attr.into(), None, None);
                                                    };
                                                } else {
                                                    ui.label(format!("(missing text attr w/ id={})", ao.1));
                                                }
                                            }

                                            ////////// Numeric //////////
                                            AttributeValueType::Numeric => {
                                                if let Some(attr) =
                                                    item.numeric_attributes.clone().iter_mut().find(|a| a.id == ao.1)
                                                {
                                                    let mut value_type = AttributeValueType::Numeric;
                                                    egui::ComboBox::from_id_salt("choice")
                                                        .width(74.0)
                                                        .selected_text(
                                                            RichText::new(value_type.to_string()).color(faded_color()).size(12.0),
                                                        )
                                                        .show_ui(ui, |ui| {
                                                            for vt in AttributeValueType::iter() {
                                                                if ui
                                                                    .selectable_value(&mut value_type, vt.clone(), vt.to_string())
                                                                    .changed()
                                                                {
                                                                    update(
                                                                        item,
                                                                        app,
                                                                        attr.into(),
                                                                        Some(value_type.clone()),
                                                                        None,
                                                                    );
                                                                    changed_value_type = true;
                                                                }
                                                            }
                                                        });

                                                    if ui
                                                        .add_sized([140.0, row_h], TextEdit::singleline(&mut attr.name))
                                                        .changed()
                                                    {
                                                        update(item, app, attr.into(), None, None);
                                                    };

                                                    let mut tmp = attr.value.to_string();
                                                    let mut cval = tmp.clone();
                                                    let id =
                                                        egui::Id::new(format!("item_{}_attr_{}_mumeric_value", item.id, attr.name));
                                                    if let Some(v) = ui.ctx().data(|d| d.get_temp::<String>(id)) {
                                                        tmp = v.clone();
                                                        cval = v;
                                                    }
                                                    let mut value_input = TextEdit::singleline(&mut tmp);

                                                    if let Err(e) = Attr::validate_value(&AttributeValueType::Numeric, &cval) {
                                                        value_input = value_input.background_color(faded_red_color());
                                                        invalid_values.insert(attr.name.clone(), e);
                                                    } else {
                                                        invalid_values.remove(&attr.name);
                                                    }

                                                    let mut resp = ui.add_sized([160.0, row_h], value_input);
                                                    if let Some(e) = invalid_values.get(&attr.name) {
                                                        resp = resp.on_hover_text_at_pointer(e);
                                                    }
                                                    if resp.changed() {
                                                        ui.ctx().data_mut(|d| d.insert_temp(id, tmp.clone()));
                                                    }
                                                    if resp.lost_focus() {
                                                        update(item, app, attr.into(), None, Some(tmp.clone()));
                                                    }
                                                } else {
                                                    ui.label(format!("(missing numeric attr w/ id={})", ao.1));
                                                }
                                            }

                                            ////////// Boolean //////////
                                            AttributeValueType::Boolean => {
                                                if let Some(attr) =
                                                    item.boolean_attributes.clone().iter_mut().find(|a| a.id == ao.1)
                                                {
                                                    let mut value_type = AttributeValueType::Boolean;
                                                    egui::ComboBox::from_id_salt("choice")
                                                        .width(74.0)
                                                        .selected_text(
                                                            RichText::new(value_type.to_string()).color(faded_color()).size(12.0),
                                                        )
                                                        .show_ui(ui, |ui| {
                                                            for vt in AttributeValueType::iter() {
                                                                if ui
                                                                    .selectable_value(&mut value_type, vt.clone(), vt.to_string())
                                                                    .changed()
                                                                {
                                                                    update(
                                                                        item,
                                                                        app,
                                                                        attr.into(),
                                                                        Some(value_type.clone()),
                                                                        None,
                                                                    );
                                                                    changed_value_type = true;
                                                                }
                                                            }
                                                        });

                                                    if ui
                                                        .add_sized([140.0, row_h], TextEdit::singleline(&mut attr.name))
                                                        .changed()
                                                    {
                                                        update(item, app, attr.into(), None, None);
                                                    };
                                                    if ui.checkbox(&mut attr.value, "").changed() {
                                                        update(item, app, attr.into(), None, Some(attr.value.to_string()));
                                                    }
                                                    let (rect, _) =
                                                        ui.allocate_exact_size(egui::vec2(180.0, row_h), egui::Sense::hover());
                                                    ui.scope_builder(
                                                        egui::UiBuilder::new()
                                                            .max_rect(rect)
                                                            .layout(egui::Layout::left_to_right(egui::Align::Center)),
                                                        |ui| {
                                                            ui.with_layout(
                                                                egui::Layout::left_to_right(egui::Align::Center),
                                                                |ui| {
                                                                    ui.label(
                                                                        egui::RichText::new(format!("({})", attr.value))
                                                                            .color(faded_color())
                                                                            .size(12.0),
                                                                    );
                                                                },
                                                            );
                                                        },
                                                    );
                                                } else {
                                                    ui.label(format!("(missing boolean attr w/id={})", ao.1));
                                                }
                                            }

                                            ////////// Date //////////
                                            AttributeValueType::Date => {
                                                if let Some(attr) = item.date_attributes.clone().iter_mut().find(|a| a.id == ao.1)
                                                {
                                                    let mut value_type = AttributeValueType::Date;
                                                    egui::ComboBox::from_id_salt("choice")
                                                        .width(74.0)
                                                        .selected_text(
                                                            RichText::new(value_type.to_string()).color(faded_color()).size(12.0),
                                                        )
                                                        .show_ui(ui, |ui| {
                                                            for vt in AttributeValueType::iter() {
                                                                if ui
                                                                    .selectable_value(&mut value_type, vt.clone(), vt.to_string())
                                                                    .changed()
                                                                {
                                                                    update(
                                                                        item,
                                                                        app,
                                                                        attr.into(),
                                                                        Some(value_type.clone()),
                                                                        None,
                                                                    );
                                                                    changed_value_type = true;
                                                                }
                                                            }
                                                        });

                                                    if ui
                                                        .add_sized([140.0, row_h], TextEdit::singleline(&mut attr.name))
                                                        .changed()
                                                    {
                                                        update(item, app, attr.into(), None, None);
                                                    };

                                                    let mut tmp = attr.value.to_string();
                                                    let mut cval = tmp.clone();
                                                    let id =
                                                        egui::Id::new(format!("item_{}_attr_{}_date_value", item.id, attr.name));
                                                    if let Some(v) = ui.ctx().data(|d| d.get_temp::<String>(id)) {
                                                        tmp = v.clone();
                                                        cval = v;
                                                    }
                                                    let mut value_input = TextEdit::singleline(&mut tmp);

                                                    if let Err(e) = Attr::validate_value(&AttributeValueType::Date, &cval) {
                                                        value_input = value_input.background_color(faded_red_color());
                                                        invalid_values.insert(attr.name.clone(), e);
                                                    } else {
                                                        invalid_values.remove(&attr.name);
                                                    }

                                                    let mut resp = ui.add_sized([160.0, row_h], value_input);
                                                    if let Some(e) = invalid_values.get(&attr.name) {
                                                        resp = resp.on_hover_text_at_pointer(e);
                                                    }
                                                    if resp.changed() {
                                                        ui.ctx().data_mut(|d| d.insert_temp(id, tmp.clone()));
                                                    }
                                                    if resp.lost_focus() {
                                                        update(item, app, attr.into(), None, Some(tmp.clone()));
                                                    }
                                                } else {
                                                    ui.label(format!("(missing date attr w/id={})", ao.1));
                                                }
                                            }

                                            ////////// DateTime //////////
                                            AttributeValueType::DateTime => {
                                                if let Some(attr) =
                                                    item.datetime_attributes.clone().iter_mut().find(|a| a.id == ao.1)
                                                {
                                                    let mut value_type = AttributeValueType::DateTime;
                                                    egui::ComboBox::from_id_salt("choice")
                                                        .width(74.0)
                                                        .selected_text(
                                                            RichText::new(value_type.to_string()).color(faded_color()).size(12.0),
                                                        )
                                                        .show_ui(ui, |ui| {
                                                            for vt in AttributeValueType::iter() {
                                                                if ui
                                                                    .selectable_value(&mut value_type, vt.clone(), vt.to_string())
                                                                    .changed()
                                                                {
                                                                    update(
                                                                        item,
                                                                        app,
                                                                        attr.into(),
                                                                        Some(value_type.clone()),
                                                                        None,
                                                                    );
                                                                    changed_value_type = true;
                                                                }
                                                            }
                                                        });

                                                    if ui
                                                        .add_sized([140.0, row_h], TextEdit::singleline(&mut attr.name))
                                                        .changed()
                                                    {
                                                        update(item, app, attr.into(), None, None);
                                                    };

                                                    let mut tmp = attr.value.to_string();
                                                    let mut cval = tmp.clone();
                                                    let id = egui::Id::new(format!(
                                                        "item_{}_attr_{}_datetime_value",
                                                        item.id, attr.name
                                                    ));
                                                    if let Some(v) = ui.ctx().data(|d| d.get_temp::<String>(id)) {
                                                        tmp = v.clone();
                                                        cval = v;
                                                    }
                                                    let mut value_input = TextEdit::singleline(&mut tmp);

                                                    if let Err(e) = Attr::validate_value(&AttributeValueType::DateTime, &cval) {
                                                        value_input = value_input.background_color(faded_red_color());
                                                        invalid_values.insert(attr.name.clone(), e);
                                                    } else {
                                                        invalid_values.remove(&attr.name);
                                                    }
                                                    let mut resp = ui.add_sized([160.0, row_h], value_input);
                                                    if let Some(e) = invalid_values.get(&attr.name) {
                                                        resp = resp.on_hover_text_at_pointer(e);
                                                    }
                                                    if resp.changed() {
                                                        ui.ctx().data_mut(|d| d.insert_temp(id, tmp.clone()));
                                                    }
                                                    if resp.lost_focus() {
                                                        update(item, app, attr.into(), None, Some(tmp.clone()));
                                                    }
                                                } else {
                                                    ui.label(format!("(missing datetime attr w/id={})", ao.1));
                                                }
                                            }
                                        }

                                        // Reserve space for the overlay X so we don't cover content.
                                        ui.add_space(row_h * 2.0);
                                    },
                                );

                                let row_rect = row.response.rect;
                                let hovered_row = ui.rect_contains_pointer(row_rect);

                                if hovered_row {
                                    let lbl_size = egui::vec2(row_h - 3.0, row_h - 4.0);
                                    let lbl_rect = egui::Rect::from_min_size(
                                        egui::pos2(row_rect.right() - lbl_size.x * 2.2, row_rect.center().y - lbl_size.y * 0.6),
                                        lbl_size,
                                    );
                                    // Place the label at a fixed rect within the row (overlay).
                                    ui.scope_builder(egui::UiBuilder::new().max_rect(lbl_rect), |ui| {
                                        handle.ui(ui, |ui| {
                                            ui.add_sized(
                                                [10.0, row_h],
                                                egui::Label::new(RichText::new(ICON_REORDER).color(faded_color()).size(9.0)),
                                            )
                                            .on_hover_cursor(CursorIcon::Crosshair)
                                            .on_hover_and_drag_cursor(CursorIcon::ResizeVertical);
                                        });
                                    });

                                    let btn_size = egui::vec2(row_h - 3.0, row_h - 4.0);
                                    let btn_rect = egui::Rect::from_min_size(
                                        egui::pos2(row_rect.right() - btn_size.x, row_rect.center().y - btn_size.y * 0.6),
                                        btn_size,
                                    );
                                    // Place the button at a fixed rect within the row (overlay).
                                    ui.scope_builder(egui::UiBuilder::new().max_rect(btn_rect), |ui| {
                                        ui.set_clip_rect(row_rect);

                                        if ui
                                            .add(
                                                Button::new(RichText::new(ICON_X_DEL).size(9.0).color(faded_color()))
                                                    .corner_radius(999.0)
                                                    .fill(egui::Color32::TRANSPARENT)
                                                    .stroke(Stroke::NONE),
                                            )
                                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                                            .clicked()
                                        {
                                            to_remove = Some((ao.0.clone(), ao.1.clone()));
                                        }
                                    });
                                }
                            }); // end-of-ui.horizontal(_)

                            ui.add_space(2.0);
                        });
                    },
                );
            });
        },
    );

    // Apply removal after list render (avoid mutating while egui_dnd iterates).
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

fn update(item: &mut Item, app: &mut CogsApp, attr: Attr, value_type: Option<AttributeValueType>, value: Option<String>) {
    let mut a = attr.clone();

    if let Some(vt) = value_type {
        item.change_attr_value_type(&a, &vt);
        if let Some(v) = value {
            if let Err(e) = Attr::validate_value(&vt, &v) {
                log::error!(
                    "Failed to validate value {} for attr={:?} to item id='{}': {}",
                    &v,
                    attr,
                    item.id,
                    e
                );
            } else {
                a.value = v;
                log::debug!("Updated attr={:?} on item id='{}' based on value_type and value.", a, item.id);
                item.change_attr_value_type(&attr.clone().into(), &vt);
            }
        }
    } else if let Some(v) = value {
        a.value = v;
        log::debug!("Updated attr={:?} on item id='{}' based on value.", a, item.id);
    }

    if let Err(e) = item.update_attribute(&a) {
        log::error!(
            "Failed to update item attributes on item id='{}' based on attr={:?}: {}",
            item.id,
            a,
            e
        );
    } else {
        // Reflect the change in the global state.
        app.state.explore.open_windows_item.insert(item.id.clone(), item.clone());
    }
}
