use crate::{
    CogsApp,
    comps::AppComponent,
    constants::{MODAL_BTN_LABEL, MODAL_BTN_MSG, MODAL_CONTENT, MODAL_TITLE},
    messages::UiMessage,
};
use egui::{Id, RichText, Shadow, Stroke};

pub struct Modal {}

impl AppComponent for Modal {
    type Context = CogsApp;

    /// Show a modal.\
    ///
    /// **Note:** It expects 4 `String` values in `ui`'s `.data` under the following `Id`s:
    /// - `MODAL_TITLE` - The title of the modal.
    /// - `MODAL_CONTENT` - The content of the modal.
    /// - `MODAL_BTN_LABEL` - The label of the button (that acknowledges and closes the modal).
    /// - `MODAL_BTN_MSG` - The message that is sent when the button is clicked.
    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        //
        let title = ui.data(|data| data.get_temp::<String>(Id::from(MODAL_TITLE))).unwrap();
        let content = ui.data(|data| data.get_temp::<String>(Id::from(MODAL_CONTENT))).unwrap();
        let btn_label = ui.data(|data| data.get_temp::<String>(Id::from(MODAL_BTN_LABEL))).unwrap();
        let btn_msg = ui.data(|data| data.get_temp::<UiMessage>(Id::from(MODAL_BTN_MSG))).unwrap();

        let frame = egui::Frame::new()
            .corner_radius(6.0)
            .inner_margin(20.0)
            .stroke(Stroke::new(1.0, ui.style().visuals.faint_bg_color))
            .fill(ui.style().visuals.window_fill)
            .shadow(Shadow::NONE);

        let _modal = egui::Modal::new(Id::new("cogs_modal")).frame(frame).show(ui.ctx(), |ui| {
            //
            ui.set_width(280.0);
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new(title).size(14.0));
            });
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.label(content);
            });

            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                if ui.button(btn_label).clicked() {
                    ctx.sendr.send(btn_msg).unwrap();
                }
            })
        });
    }
}
