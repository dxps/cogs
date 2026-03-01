use crate::{CogsApp, comps::item::ItemWindowState};
use cogs_shared::domain::model::meta::{AttributeValueType, Item};
use egui::{ComboBox, Label, TextEdit, Ui};
use strum::IntoEnumIterator;

pub(super) fn row_add_attr(app: &mut CogsApp, ui: &mut Ui, element: &mut Item, state: &mut ItemWindowState<'_>) {
    ui.add_enabled(false, Label::new("      Add Attribute"));
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
        ui.add_enabled(false, Label::new(" Name"));
        if ui
            .add_sized([180.0, ui.spacing().interact_size.y], TextEdit::singleline(&mut name))
            .changed()
        {
            app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap().name = name.clone();
        }
    });
    ui.end_row();
    ui.label("");
    ui.horizontal(|ui| {
        ui.add_enabled(false, Label::new("   Type"));
        ComboBox::from_id_salt(format!("at_val_type_{}", state.id))
            .width(178.0)
            .selected_text(if let Some(vt) = &value_type {
                vt.to_string()
            } else {
                String::new()
            })
            .show_ui(ui, |ui| {
                for vb in AttributeValueType::iter() {
                    if ui
                        .selectable_value(&mut value_type, Some(vb.clone()), vb.to_string())
                        .changed()
                    {
                        log::debug!("On new item, selected value_type: {value_type:?}");
                        if let Some(vt) = value_type.clone() {
                            app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap().value_type = vt;
                        }
                    };
                }
            });
    });
    ui.end_row();
    ui.label("");
    ui.horizontal(|ui| {
        ui.add_enabled(false, Label::new(" Value"));
        if ui
            .add_sized([180.0, ui.spacing().interact_size.y], TextEdit::singleline(&mut value))
            .changed()
        {
            app.state.explore.item_cu_add_attr.get_mut(&state.id).unwrap().value = value;
        }
    });
    ui.end_row();
}
