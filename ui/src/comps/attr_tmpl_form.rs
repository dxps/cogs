use crate::{CogsApp, ManagedAttrTemplate, comps::AppComponent};
use cogs_shared::domain::model::meta::AttributeValueType;
use egui::{Align, ComboBox, Grid, Layout, RichText, Ui, Window};

pub struct AttrTemplateForm {}

impl AppComponent for AttrTemplateForm {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ui: &mut Ui) {
        //
        let title = "New Attribute Template";
        Window::new("AttrTemplateForm")
            .title_bar(false)
            .resizable(false)
            .min_width(300.0)
            .max_width(400.0)
            .min_height(200.0)
            .max_height(400.0)
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(title).size(13.0));
                        });
                    });
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        Grid::new("manage_attr_templ")
                            .spacing([10.0, 10.0])
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.label("            Name");
                                ui.text_edit_singleline(&mut ctx.state.data.curr_attr_template.name);
                                ui.end_row();
                                ui.label("   Description");
                                ui.text_edit_singleline(&mut ctx.state.data.curr_attr_template.description);
                                ui.end_row();
                                ui.label("    Value Type");
                                ComboBox::from_id_salt("attr_templ_val_type")
                                    .width(287.0)
                                    .selected_text(ctx.state.data.curr_attr_template.value_type.to_string())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut ctx.state.data.curr_attr_template.value_type,
                                            AttributeValueType::Text,
                                            AttributeValueType::Text.to_string(),
                                        );
                                        ui.selectable_value(
                                            &mut ctx.state.data.curr_attr_template.value_type,
                                            AttributeValueType::SmallInteger,
                                            AttributeValueType::SmallInteger.to_string(),
                                        );
                                    });
                                ui.end_row();
                                ui.label("Default value");
                                ui.text_edit_singleline(&mut ctx.state.data.curr_attr_template.default_value);
                                ui.end_row();
                                ui.label("    Mandatory");
                                ui.checkbox(&mut ctx.state.data.curr_attr_template.is_required, "");
                                ui.end_row();
                            });
                        ui.add_space(8.0);
                    });

                    ui.add_space(4.0);

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.add_space(18.0);
                        if ui.button("    Save    ").clicked() {
                            ctx.state.data.save_attr_template(ui.ctx(), ctx.sendr.clone());
                        }
                        ui.add_space(8.0);
                        if ui.button("  Cancel  ").clicked() {
                            ctx.state.data.curr_attr_template = ManagedAttrTemplate::default();
                            ctx.state.explore.add_kind = None;
                        }
                    });
                    ui.add_space(12.0);
                });
            });
    }
}
