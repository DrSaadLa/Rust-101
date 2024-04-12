pub fn center(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    if width <= text_len {
        return text.to_string();
    }
    let total_padding = width - text_len;
    let pad_left = total_padding / 2;
    let pad_right = total_padding - pad_left-2;

    // Construct the centered string with padding
    format!("{:pad_left$}{text}{:pad_right$}", "", "", pad_left=pad_left, pad_right=pad_right, text=text)
}