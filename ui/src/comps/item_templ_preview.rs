use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ELEMENT};
use cogs_shared::domain::model::meta::ItemTemplate;
use egui::{Align, FontId, Grid, Label, Layout, TextStyle};

pub struct ItemTemplatePreview {}

impl AppComponent for ItemTemplatePreview {
    type Context = CogsApp;

    /// It shows the properties of an item template.
    /// It expects to get the item template in `ui`'s `.data()` key named `EXPLORE_ELEMENT`.
    fn show(_: &mut Self::Context, ui: &mut egui::Ui) {
        let element = ui
            .ctx()
            .data(|d| d.get_temp::<ItemTemplate>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();

        ui.scope(|ui| {
            // Apply once for everything inside this scope.
            let font = FontId::proportional(12.0);
            ui.style_mut().text_styles.insert(TextStyle::Body, font.clone());

            Grid::new("explore_curr_elem_preview").num_columns(2).show(ui, |ui| {
                // name
                ui.add_enabled(false, Label::new("name"));
                ui.add(Label::new(element.name.as_str()));
                ui.end_row();

                // description
                ui.add_enabled(false, Label::new("description"));
                ui.add(Label::new(element.description.as_str()));
                ui.end_row();

                // listing attribute
                ui.add_enabled(false, Label::new("listing attribute"));
                ui.add(Label::new(element.listing_attr.name.as_str()));
                ui.end_row();

                // attributes (top-aligned label)
                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    ui.add_enabled(false, Label::new("attributes"));
                });

                let attrs_str = element
                    .attributes
                    .iter()
                    .map(|a| a.name.clone())
                    .collect::<Vec<String>>()
                    .join("\n");

                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    ui.add(Label::new(attrs_str));
                });
                ui.end_row();

                // links (top-aligned label)
                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    ui.add_enabled(false, Label::new("links"));
                });

                let links_str = element
                    .links
                    .iter()
                    .map(|l| l.name.clone())
                    .collect::<Vec<String>>()
                    .join("\n");

                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    ui.add(Label::new(links_str));
                });
                ui.end_row();
            });
        });
    }
}
