pub fn text_up_to_lenght(text: &str, max_length: usize) -> String {
    if text.len() > max_length {
        format!("{}...", &text[0..max_length - 4])
    } else {
        // Pad with spaces on the right up to max_length
        format!("{text:<width$}", width = max_length)
    }
}

pub fn strong_separator(ui: &mut egui::Ui, width: f32) {
    let desired_h = 8.0; // includes vertical breathing room
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, desired_h), egui::Sense::hover());

    let y = rect.center().y;
    let x0 = rect.left();
    let x1 = rect.right();

    let color = ui.visuals().extreme_bg_color;
    let stroke = egui::Stroke::new(1.0, color);

    ui.painter().line_segment([egui::pos2(x0, y), egui::pos2(x1, y)], stroke);
}
