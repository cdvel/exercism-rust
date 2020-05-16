extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation as uc;

pub fn reverse(input: &str) -> String {
    // input.chars().rev().collect::<String>().to_string()
    // grapheme aware solution
    uc::graphemes(input, true)
        .rev()
        .collect::<Vec<&str>>()
        .join("")
}
