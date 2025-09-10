use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ELEMENT};
use cogs_shared::domain::model::meta::ItemTemplate;
use egui::{Color32, Grid, RichText};

pub struct ItemTemplateProps {}

impl AppComponent for ItemTemplateProps {
    type Context = CogsApp;

    /// It shows the properties of an item template.
    /// It expects to get the item template in `ui`'s `.data()` key named `EXPLORE_ELEMENT`.
    fn show(_: &mut Self::Context, ui: &mut egui::Ui) {
        //
        let element = ui
            .ctx()
            .data(|d| d.get_temp::<ItemTemplate>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();

        Grid::new("explore_curr_elem_props").num_columns(2).show(ui, |ui| {
            ui.label(RichText::new("name:").color(Color32::GRAY));
            ui.label(element.name);
            ui.end_row();

            ui.label(RichText::new("description:").color(Color32::GRAY));
            ui.label(element.description);
            ui.end_row();

            // TODO: Show the rest: its attributes and the listing one.

            ui.label(RichText::new("id:").color(Color32::GRAY));
            ui.label(RichText::new(element.id.to_string()).color(Color32::GRAY).size(10.0));
        });
    }
}
