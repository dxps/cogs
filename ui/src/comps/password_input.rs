use crate::{CogsApp, comps::AppComponent};

pub struct PasswordInput {}

impl AppComponent for PasswordInput {
    type Context = CogsApp;

    fn show_input_entered(ui: &mut egui::Ui, value: &mut String, entered: &mut bool) {
        show_password_input(ui, value, entered);
    }
}

// FYI: This code is from https://github.com/egui-rs/egui/blob/master/egui_demo_lib/src/demo/password_input.rs.
fn show_password_input(ui: &mut egui::Ui, pass: &mut String, entered: &mut bool) -> egui::Response {
    // This widget has its own state — show or hide password characters (`show_plaintext`).
    // In this case we use a simple `bool`, but you can also declare your own type.
    // It must implement at least `Clone` and be `'static`.
    // If you use the `persistence` feature, it also must implement `serde::{Deserialize, Serialize}`.

    // Generate an id for the state
    let state_id = ui.id().with("show_plaintext");

    // Get state for this widget.
    // You should get state by value, not by reference to avoid borrowing of [`Memory`].
    let mut show_plaintext = ui.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

    // Process ui, change a local copy of the state.
    // We want TextEdit to fill entire space, and have button after that,
    // so in that case we can change direction to right_to_left.
    let result = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        // Toggle the `show_plaintext` bool with a button:
        let response = ui.selectable_label(show_plaintext, "👁").on_hover_text("Show/hide password");

        if response.clicked() {
            show_plaintext = !show_plaintext;
        }

        // Show the password field.
        let input = egui::TextEdit::singleline(pass).password(!show_plaintext);
        let resp = ui.add_sized(ui.available_size(), input);
        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            log::info!("[show_password_input] Enter pressed on password field");
            *entered = true;
        }
    });

    // Store the (possibly changed) state:
    ui.data_mut(|d| d.insert_temp(state_id, show_plaintext));

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, …) and maybe show a tooltip:
    result.response
}
