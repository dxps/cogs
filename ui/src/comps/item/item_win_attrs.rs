use crate::{CogsApp, comps::item::ItemWindowState};
use cogs_shared::domain::model::{
    Id,
    meta::{AttributeValueType, Item},
};
use egui::{Button, CollapsingHeader, ComboBox, CursorIcon, Grid, Label, TextEdit, Ui};
use strum::IntoEnumIterator;

pub(super) fn render_attrs(app: &mut CogsApp, ui: &mut egui::Ui, item: &mut Item, _state: &mut ItemWindowState) {
    let row_h = 20.0;
    let list_h = 260.0;

    // Work on a temporary order that we can reorder via DnD:
    let mut order = item.attributes_order.clone();
    log::debug!("order: {order:?} as per item.attributes_order: {:?}", item.attributes_order);
    let mut changed_value_type = false;

    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), list_h),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            egui::ScrollArea::vertical().auto_shrink([true; 2]).show(ui, |ui| {
                egui_dnd::dnd(ui, "rows_dnd").show_vec(
                    &mut order,
                    |ui, ao: &mut (AttributeValueType, Id), handle, _item_state| {
                        ui.push_id(
                            &ao.1, // Stable id for this row.
                            |ui| {
                                let row_w = ui.available_width();
                                ui.allocate_ui_with_layout(
                                    egui::vec2(row_w, row_h),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        handle.ui(ui, |ui| {
                                            ui.add_sized([20.0, row_h], egui::Label::new("⠿"));
                                        });
                                        match ao.0 {
                                            AttributeValueType::Text => {
                                                if let Some(attr) = item.text_attributes.clone().iter_mut().find(|a| a.id == ao.1)
                                                {
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
                                                                    // Reflect the change in the global state.
                                                                    app.state
                                                                        .explore
                                                                        .open_windows_item
                                                                        .insert(item.id.clone(), item.clone());
                                                                    log::debug!(
                                                                        "[changed text attr] Updated open_windows_item: {:?}",
                                                                        app.state.explore.open_windows_item
                                                                    );
                                                                };
                                                            }
                                                        });

                                                    ui.add_sized([140.0, row_h], egui::TextEdit::singleline(&mut attr.value));
                                                } else {
                                                    ui.label(format!("(missing text) for ao.id={}", ao.1));
                                                }
                                            }
                                            AttributeValueType::Numeric => {
                                                if let Some(attr) =
                                                    item.numeric_attributes.clone().iter_mut().find(|a| a.id == ao.1)
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
                                                                    // Reflect the change in the global state.
                                                                    app.state
                                                                        .explore
                                                                        .open_windows_item
                                                                        .insert(item.id.clone(), item.clone());
                                                                    log::debug!(
                                                                        "[changed numeric attr] Updated open_windows_item: {:?}",
                                                                        app.state.explore.open_windows_item
                                                                    );
                                                                };
                                                            }
                                                        });

                                                    ui.add_sized(
                                                        [140.0, row_h],
                                                        egui::TextEdit::singleline(&mut attr.value.to_string()),
                                                    );
                                                } else {
                                                    ui.label(format!("(missing numeric) for a.id={}", ao.1));
                                                }
                                            }
                                            _ => {
                                                ui.label("(not implemented)");
                                            }
                                        }
                                    },
                                );
                                ui.add_space(2.0); // vertical gap between rows
                            },
                        );
                    },
                );
            });
        },
    );

    // Persist reordered list back onto the item, but only if it's a DnD result.
    if order != item.attributes_order && !changed_value_type {
        item.attributes_order = order;
        // Reflect the change in the global state.
        app.state.explore.open_windows_item.insert(item.id.clone(), item.clone());
        log::debug!(
            "[changed order] Updated open_windows_item: {:?}",
            app.state.explore.open_windows_item
        );
    }
}

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
