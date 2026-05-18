use crate::{
    comps::AppComponent,
    constants::{EXPLORE_ELEMENT, ICON_USER},
    utils::strong_separator,
    CogsApp,
};
use cogs_shared::domain::model::AccessLevel;
use egui::{CursorIcon, FontId, Grid, Label, RichText, TextStyle};

pub struct AccessLevelPreview {}

impl AppComponent for AccessLevelPreview {
    type Context = CogsApp;

    /// It shows the properties of an access level.
    /// It expects to get the access level in `ui`'s `.data()` key named `EXPLORE_ELEMENT`.
    fn show(_: &mut Self::Context, ui: &mut egui::Ui) {
        let element = ui
            .ctx()
            .data(|d| d.get_temp::<AccessLevel>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();

        ui.label(format!("{} {}", ICON_USER, element.name.as_str()))
            .on_hover_cursor(CursorIcon::Help)
            .on_hover_text("This is an access level.");

        ui.add_space(4.0);
        strong_separator(ui, ui.available_width());
        ui.add_space(2.0);

        ui.scope(|ui| {
            let base = FontId::proportional(12.0);
            ui.style_mut().text_styles.insert(TextStyle::Body, base);

            Grid::new("explore_curr_elem_preview").num_columns(2).show(ui, |ui| {
                ui.add_enabled(false, Label::new(RichText::new("description")));
                ui.add(Label::new(element.description.as_deref().unwrap_or_default()));
                ui.end_row();
            });

            ui.add_space(4.0);
            strong_separator(ui, ui.available_width());
        });
    }
}
