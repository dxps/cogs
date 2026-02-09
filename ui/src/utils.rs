pub fn text_up_to_lenght(text: &str, max_length: usize) -> String {
    if text.len() > max_length {
        format!("{}...", &text[0..max_length - 4])
    } else {
        // Pad with spaces on the right up to max_length
        format!("{text:<width$}", width = max_length)
    }
}
