use crate::{measure::text_size, superfont::SuperFont};

#[cfg(feature = "emoji")]
use crate::measure::text_size_with_emojis;

pub fn word_wrap(text: &str, width: i32, font: &SuperFont, scale: rusttype::Scale) -> Vec<String> {
    // word wrap based on pixel width
    let mut result = Vec::new();

    for line_r in text.split("\n") {
        let mut line = String::new();
        for word in line_r.split_whitespace() {
            let mut new_line = line.clone();
            if !new_line.is_empty() {
                new_line.push(' ');
            }
            new_line.push_str(word);

            let w = text_size(scale, font, &new_line).0;

            if w > width {
                if !line.is_empty() {
                    result.push(line);
                }
                line = word.to_string();
            } else {
                line = new_line;
            }
        }

        line = line.trim().to_string();

        if !line.is_empty() {
            result.push(line);
        }
    }

    result
}

#[cfg(feature = "emoji")]
pub fn word_wrap_with_emojis(
    text: &str,
    width: i32,
    font: &SuperFont,
    scale: rusttype::Scale,
) -> Vec<String> {
    // word wrap based on pixel width
    let mut result = Vec::new();

    for line_r in text.split("\n") {
        let mut line = String::new();
        for word in line_r.split_whitespace() {
            let mut new_line = line.clone();
            if !new_line.is_empty() {
                new_line.push(' ');
            }
            new_line.push_str(word);

            let w = text_size_with_emojis(scale, font, &new_line).0;

            if w > width {
                if !line.is_empty() {
                    result.push(line);
                }
                line = word.to_string();
            } else {
                line = new_line;
            }
        }

        line = line.trim().to_string();

        if !line.is_empty() {
            result.push(line);
        }
    }

    result
}
