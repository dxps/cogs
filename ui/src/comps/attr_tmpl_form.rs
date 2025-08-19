use crate::{CogsApp, ManagedAttrTemplate, comps::AppComponent};
use cogs_shared::domain::model::{
    Id,
    meta::{AttributeValueType, Kind},
};
use egui::{Align, Color32, ComboBox, Grid, Layout, RichText, Window};

pub struct AttrTemplateForm {}

impl AppComponent for AttrTemplateForm {
    type Context = CogsApp;

    /// Show the form for creating or editing an attribute template.
    /// As params, it looks for an `AttrTemplate` in `ectx`'s data key `attr_template`.
    /// If nothing found, then it considers it's the new attribute template use case.
    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        //
        let ectx = ui.ctx();
        let id: Id;
        let title: &str;
        let mut element = ManagedAttrTemplate::default();

        match ectx.data(|d| d.get_temp::<ManagedAttrTemplate>(egui::Id::from("attr_template"))) {
            Some(at) => {
                id = at.id.clone();
                if id.is_zero() {
                    title = "New Attribute Template";
                } else {
                    element = at.into();
                    title = "Edit Attribute Template";
                }
            }
            None => {
                id = Id::default();
                title = "New Attribute Template";
            }
        };

        Window::new(format!("AttrTemplateForm_id_{}", id))
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

                    ui.add_space(4.0);

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.add_space(18.0);
                        if ui.button("    Save    ").clicked() {
                            ctx.state.data.save_attr_template(element, ui.ctx(), ctx.sendr.clone());
                        }
                        ui.add_space(8.0);
                        if ui.button("  Cancel  ").clicked() {
                            // ctx.state.data.curr_attr_template = ManagedAttrTemplate::default();
                            // ctx.state.explore.add_kind = None;
                            ctx.state.explore.open_windows.remove(&(Kind::AttributeTemplate, id));
                        }
                    });
                    ui.add_space(12.0);
                });
            });
    }
}
