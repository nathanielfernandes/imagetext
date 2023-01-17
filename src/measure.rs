use crate::superfont::SuperFont;

pub fn text_size(scale: rusttype::Scale, font: &SuperFont, text: &str) -> (i32, i32) {
    let (mut w, mut h) = (0, 0);

    for g in font.layout(text, scale, rusttype::point(0.0, 0.0)) {
        if let Some(bb) = g.pixel_bounding_box() {
            w = std::cmp::max(w, bb.max.x);
            h = std::cmp::max(h, bb.max.y);
        }
    }

    (w, h)
}

pub fn text_size_multiline(
    lines: &Vec<String>,
    font: &SuperFont,
    scale: rusttype::Scale,
    line_spacing: f32,
) -> (i32, i32) {
    let mut width = 0;

    for line in lines {
        width = width.max(text_size(scale, font, line).0);
    }

    (
        width,
        ((lines.len() as f32 * scale.y * line_spacing) - (line_spacing - 1.0) * scale.y).round()
            as i32,
    )
}
