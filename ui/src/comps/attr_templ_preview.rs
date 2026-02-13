use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ELEMENT};
use cogs_shared::domain::model::meta::AttrTemplate;
use egui::{FontId, Grid, Label, RichText, TextStyle};

pub struct AttrTemplatePreview {}

impl AppComponent for AttrTemplatePreview {
    type Context = CogsApp;

    /// It shows the properties of an attribute template.
    /// It expects to get the attribute template in `ui`'s `.data()` key named `EXPLORE_ELEMENT`.
    fn show(_: &mut Self::Context, ui: &mut egui::Ui) {
        let element = ui
            .ctx()
            .data(|d| d.get_temp::<AttrTemplate>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();

        ui.scope(|ui| {
            // Set font sizes once for this whole preview scope
            let base = FontId::proportional(12.0);
            ui.style_mut().text_styles.insert(TextStyle::Body, base.clone());

            Grid::new("explore_curr_elem_props").num_columns(2).show(ui, |ui| {
                ui.add_enabled(false, Label::new(RichText::new("name")));
                ui.add(Label::new(element.name.as_str()));
                ui.end_row();

                ui.add_enabled(false, Label::new(RichText::new("description")));
                ui.add(Label::new(element.description.as_str()));
                ui.end_row();

                ui.add_enabled(false, Label::new(RichText::new("value type")));
                ui.add(Label::new(element.value_type.to_string()));
                ui.end_row();

                ui.add_enabled(false, Label::new(RichText::new("default value")));
                ui.add(Label::new(element.default_value.as_str()));
                ui.end_row();

                ui.add_enabled(false, Label::new(RichText::new("is required")));
                ui.add(Label::new(element.is_required.to_string()));
                ui.end_row();

                ui.add_enabled(false, Label::new(RichText::new("type")));
                ui.add(Label::new(RichText::new("attribute template")));
                ui.end_row();
            });
        });
    }
}
