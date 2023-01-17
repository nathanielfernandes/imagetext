use crate::{measure::text_size, superfont::SuperFont};

// rough port of https://github.com/fogleman/gg/blob/master/wrap.go#L12
// TODO: rewrite this
pub fn split_on_space(x: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut pi = 0;
    let mut ps = false;
    for (i, c) in x.char_indices() {
        let s = c.is_whitespace();
        if s != ps && i > 0 {
            result.push(&x[pi..i]);
            pi = i;
        }
        ps = s;
    }
    result.push(&x[pi..]);
    result
}

// rough port of https://github.com/fogleman/gg/blob/master/wrap.go#L28
// TODO: rewrite this
pub fn word_wrap(text: &str, width: i32, font: &SuperFont, scale: rusttype::Scale) -> Vec<String> {
    let mut lines = Vec::new();

    for line in text.split("\n") {
        let mut fields: Vec<&str> = split_on_space(line);

        if fields.len() % 2 == 1 {
            fields.push("");
        }

        let mut x = String::new();
        for i in (0..fields.len()).step_by(2) {
            let w = text_size(scale, font, &(x.clone() + fields[i])).0;

            if w > width {
                if x.is_empty() {
                    lines.push(fields[i].to_string());
                    continue;
                } else {
                    lines.push(x.clone());
                    x = String::new();
                }
            }
            x += &(fields[i].to_owned() + fields[i + 1]);
        }

        if !x.is_empty() {
            lines.push(x);
        }
    }

    for line in &mut lines {
        *line = line.trim().to_string();
    }

    lines
}
