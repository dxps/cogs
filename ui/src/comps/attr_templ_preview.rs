use crate::{
    CogsApp,
    comps::AppComponent,
    constants::{EXPLORE_ELEMENT, ICON_ATTR_TMPL},
    utils::strong_separator,
};
use cogs_shared::domain::model::meta::AttrTemplate;
use egui::{CursorIcon, FontId, Grid, Label, RichText, TextStyle};

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

        ui.label(format!("{} {}", ICON_ATTR_TMPL, element.name.as_str()))
            .on_hover_cursor(CursorIcon::Help)
            .on_hover_text("This is an attribute template.");

        ui.add_space(4.0);
        strong_separator(ui, ui.available_width());
        ui.add_space(2.0);

        ui.scope(|ui| {
            // Apply once for everything inside this scope.
            let base = FontId::proportional(12.0);
            ui.style_mut().text_styles.insert(TextStyle::Body, base.clone());

            Grid::new("explore_curr_elem_preview").num_columns(2).show(ui, |ui| {
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
            });

            ui.add_space(4.0);
            strong_separator(ui, ui.available_width());
        });
    }
}
