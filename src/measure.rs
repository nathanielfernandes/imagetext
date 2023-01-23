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

#[cfg(feature = "emoji")]
pub fn text_size_with_emojis(scale: rusttype::Scale, font: &SuperFont, text: &str) -> (i32, i32) {
    let (mut w, mut h) = (0, 0);
    let (text, emojis) = crate::emoji::parse::parse_out_emojis(
        text,
        font.emoji_options.allow_shortcodes,
        font.emoji_options.allow_discord,
    );

    for g in font.layout_with_emojis(
        &text,
        &emojis,
        font.emoji_options.scale,
        scale,
        rusttype::point(0.0, 0.0),
    ) {
        if let Some(bb) = g.0.pixel_bounding_box() {
            w = std::cmp::max(w, bb.max.x);
            h = std::cmp::max(h, bb.max.y);
        }
    }
    (w, h)
}

#[cfg(feature = "emoji")]
pub fn text_size_multiline_with_emojis(
    lines: &Vec<String>,
    font: &SuperFont,
    scale: rusttype::Scale,
    line_spacing: f32,
) -> (i32, i32) {
    let mut width = 0;

    for line in lines {
        width = width.max(text_size_with_emojis(scale, font, line).0);
    }

    (
        width,
        ((lines.len() as f32 * scale.y * line_spacing) - (line_spacing - 1.0) * scale.y).round()
            as i32,
    )
}
