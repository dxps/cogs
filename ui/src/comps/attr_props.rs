use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ATTR_TEMPLATE};
use cogs_shared::domain::model::meta::AttrTemplate;
use egui::{Color32, Grid, RichText};

pub struct AttrTemplateProps {}

impl AppComponent for AttrTemplateProps {
    type Context = CogsApp;

    /// It shows the properties of an attribute template.
    /// It expects to get the attribute template in `ui`'s `.data()` key named `EXPLORE_ATTR_TEMPLATE`.
    fn show(_: &mut Self::Context, ui: &mut egui::Ui) {
        //
        let element = ui
            .ctx()
            .data(|d| d.get_temp::<AttrTemplate>(egui::Id::from(EXPLORE_ATTR_TEMPLATE)))
            .clone()
            .unwrap_or_default();

        Grid::new("explore_curr_elem_props").num_columns(2).show(ui, |ui| {
            ui.label(RichText::new("name:").color(Color32::GRAY));
            ui.label(element.name);
            ui.end_row();

            ui.label(RichText::new("description:").color(Color32::GRAY));
            ui.label(element.description);
            ui.end_row();

            ui.label(RichText::new("value type:").color(Color32::GRAY));
            ui.label(element.value_type.to_string());
            ui.end_row();

            ui.label(RichText::new("default value:").color(Color32::GRAY));
            ui.label(element.default_value);
            ui.end_row();

            ui.label(RichText::new("is required:").color(Color32::GRAY));
            ui.label(element.is_required.to_string());
            ui.end_row();
            ui.label(RichText::new("type:").color(Color32::GRAY));
            ui.label(RichText::new("attribute template").color(Color32::GRAY));
            ui.end_row();
            ui.label(RichText::new("id:").color(Color32::GRAY));
            ui.label(RichText::new(element.id.to_string()).color(Color32::GRAY).size(10.0));
        });
    }
}
