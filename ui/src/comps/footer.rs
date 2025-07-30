use crate::{CogsApp, comps::AppComponent};
use egui::Ui;

pub struct Footer {}

impl AppComponent for Footer {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            ui.label(" and ");
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(". ");
            if ui
                .label("Status")
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                let req = ehttp::Request::get("http://localhost:9009/manifest.json");
                ehttp::fetch(req, move |rsp| {
                    log::info!("[status] clicked. Test response: {:#?}", rsp);
                });
            }
        });
    }
}
