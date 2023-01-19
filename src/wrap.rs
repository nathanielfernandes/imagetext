use crate::{measure::text_size, superfont::SuperFont};

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
                if line.is_empty() {
                    // word is too long to fit on a line by itself
                    // just add it to the line
                    line.push_str(word);
                } else {
                    // word is too long to fit on a line by itself
                    // add the line to the result and start a new line
                    result.push(line);
                    line = String::new();
                }
            } else {
                // word fits on the line
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
