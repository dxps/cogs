use crate::{CogsApp, views::AppView};
use egui::{Align2, RichText, Shadow, Stroke};

pub struct Login {}

impl AppView for Login {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ectx: &egui::Context) {
        //
        egui::CentralPanel::default().show(ectx, |ui| {
            let frame = egui::Frame::new()
                .corner_radius(6.0)
                .inner_margin(20.0)
                .stroke(Stroke::new(1.0, ui.style().visuals.extreme_bg_color))
                .shadow(Shadow::NONE);

            let window = egui::Window::new("")
                .id(egui::Id::new("login_window_id")) // required since we change the title
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(frame)
                .max_size((340.0, 200.0))
                .resizable(false)
                .title_bar(false);

            window.show(ectx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add_space(10.0);
                        ui.label(RichText::new("Login").heading());
                        ui.add_space(8.0);
                        ui.label("Provide the credentials below to authenticate into the system.");
                        ui.add_space(20.0);
                    });
                });

                ui.horizontal(|ui| {
                    ui.label("Username: ");
                    ui.add_space(4.0);
                    ui.text_edit_singleline(&mut ctx.state.user);
                    ui.add_space(20.0);
                });
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label(" Password: ");
                    ui.add_space(4.0);
                    // ui.text_edit_singleline(&mut ctx.state.pass);
                    ui.add(password_text_edit_singleline(&mut ctx.state.pass));
                });

                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    if ui.button("  Login  ").clicked() {
                        log::info!("Login w/ user: {} pass: {}", ctx.state.user, ctx.state.pass);
                    };
                    ui.add_space(10.0);
                });
            });
        });
    }
}

pub fn password_text_edit_singleline(pass: &mut String) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| password_ui(ui, pass)
}

fn password_ui(ui: &mut egui::Ui, pass: &mut String) -> egui::Response {
    // This widget has its own state — show or hide password characters (`show_plaintext`).
    // In this case we use a simple `bool`, but you can also declare your own type.
    // It must implement at least `Clone` and be `'static`.
    // If you use the `persistence` feature, it also must implement `serde::{Deserialize, Serialize}`.

    // Generate an id for the state
    let state_id = ui.id().with("show_plaintext");

    // Get state for this widget.
    // You should get state by value, not by reference to avoid borrowing of [`Memory`].
    let mut show_plaintext = ui.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

    // Process ui, change a local copy of the state
    // We want TextEdit to fill entire space, and have button after that, so in that case we can
    // change direction to right_to_left.
    let result = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        // Toggle the `show_plaintext` bool with a button:
        let response = ui
            .selectable_label(show_plaintext, "👁")
            .on_hover_text("Show/hide password");

        if response.clicked() {
            show_plaintext = !show_plaintext;
        }

        // Show the password field:
        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::singleline(pass).password(!show_plaintext),
        );
    });

    // Store the (possibly changed) state:
    ui.data_mut(|d| d.insert_temp(state_id, show_plaintext));

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, …) and maybe show a tooltip:
    result.response
}
