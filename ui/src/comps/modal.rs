use egui::{Id, RichText, Shadow, Stroke};

use crate::{CogsApp, comps::AppComponent};

pub struct Modal {}

impl AppComponent for Modal {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        //
        let frame = egui::Frame::new()
            .corner_radius(6.0)
            .inner_margin(20.0)
            .stroke(Stroke::new(1.0, ui.style().visuals.faint_bg_color))
            .fill(ui.style().visuals.window_fill)
            .shadow(Shadow::NONE);

        let _modal = egui::Modal::new(Id::new("cogs_modal"))
            .frame(frame)
            .show(ui.ctx(), |ui| {
                //
                ui.set_width(280.0);
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Authentication failed").size(14.0));
                });
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.label("Invalid username or password. Please try again.");
                });

                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    if ui.button("  Close  ").clicked() {
                        ctx.sendr
                            .send(crate::messages::UiMessage::Login(Ok(None)))
                            .unwrap();
                    }
                })
            });
    }
}
