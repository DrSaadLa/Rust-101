/// Centers a given text within a specified or standard width.
///
/// The function adds padding to both sides of the input text so that it appears
/// centered. If the specified width is less than the length of the text, the original
/// text is returned. If the width is more than the standard width (72 characters by default),
/// the text is centered within the standard width.
///
/// # Arguments
///
/// * `text` - The text to be centered.
/// * `width` - The desired total width of the line within which to center the text.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use your_crate::center; // Replace `your_crate` with the name of your crate
/// let text = "Rust";
/// let centered = center(text, 10);
/// assert_eq!(centered, "   Rust   ");
/// ```
///
/// Exceeding standard width:
///
/// ```
/// # use your_crate::center; // Replace `your_crate` with the name of your crate
/// let text = "Rust";
/// let centered = center(text, 80); // Standard width is 72
/// assert_eq!(centered, "                                 Rust                                 ");
/// ```
///
/// # Returns
///
/// A new `String` that represents the original text centered within the
/// specified or standard width.
pub fn center(text: &str, mut width: usize) -> String {
    const STANDARD_WIDTH: usize = 79;
    let text_len = text.chars().count();

    // If specified width is less than text length, return original text
    if width <= text_len || width > STANDARD_WIDTH {
        width = std::cmp::min(text_len, STANDARD_WIDTH);
    }

    let total_padding = width.saturating_sub(text_len);
    let pad_left = total_padding / 2;
    let pad_right = total_padding - pad_left;

    format!("{:pad_left$}{text}{:pad_right$}", "", "", pad_left=pad_left, pad_right=pad_right, text=text)
}
