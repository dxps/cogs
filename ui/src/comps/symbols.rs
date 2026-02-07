pub fn paint_combo_chevron(ui: &egui::Ui, rect: egui::Rect) {
    //
    let color = ui.visuals().widgets.inactive.fg_stroke.color;

    // Position near right side of the trigger rect
    let cx = rect.right() - 16.0;
    let cy = rect.center().y;
    let w = 5.0;
    let h = 3.0;

    let points = vec![egui::pos2(cx - w, cy - h), egui::pos2(cx + w, cy - h), egui::pos2(cx, cy + h)];

    ui.painter()
        .add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
}
