use crate::{CogsApp, comps::AppComponent};
use egui::{Color32, FontFamily, FontId, RichText, Sense, Ui, vec2};

pub struct Footer {}

impl AppComponent for Footer {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ui: &mut Ui) {
        ui.scope(|ui| {
            // Footer-wide text style.
            let dim = Color32::from_gray(140);
            let hover_orange = Color32::from_rgb(255, 165, 0);
            let font = FontId::new(11.0, FontFamily::Proportional);
            ui.style_mut().override_font_id = Some(font.clone());
            ui.visuals_mut().override_text_color = Some(dim);

            let copyright = "(c) 2026 cogs";
            let status = "Status";
            let gap = 20.0;

            // Measure text widths with the actual font
            let color = ui.visuals().text_color();
            let w1 = ui
                .painter()
                .layout_no_wrap(copyright.to_owned(), font.clone(), color)
                .size()
                .x;
            let w2 = ui.painter().layout_no_wrap(status.to_owned(), font.clone(), color).size().x;

            let content_w = w1 + gap + w2;
            let avail_w = ui.available_width();
            let left_pad = ((avail_w - content_w) * 0.5).max(0.0);

            let h = ui.spacing().interact_size.y;
            let (_rect, _) = ui.allocate_exact_size(vec2(avail_w, h), Sense::hover());

            ui.horizontal(|ui| {
                ui.add_space(left_pad);
                ui.label(copyright);
                ui.add_space(gap);

                let resp = ui
                    .add(egui::Label::new(RichText::new(status).size(font.size)).sense(Sense::click()))
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text("Click to get the status of the system.");

                // 2) repaint text color based on hover (overrides gray footer color)
                let color = if resp.hovered() { hover_orange } else { dim };
                ui.painter()
                    .text(resp.rect.left_top(), egui::Align2::LEFT_TOP, status, font.clone(), color);

                if resp.clicked() {
                    let req = ehttp::Request::get("http://localhost:9009/manifest.json");
                    ehttp::fetch(req, move |rsp| {
                        log::info!("[status] clicked. Test response: {:?}", rsp);
                    });
                }
            });
        });
    }
}
