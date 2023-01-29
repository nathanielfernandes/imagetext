use crate::superfont::SuperFont;

pub fn text_size(scale: rusttype::Scale, font: &SuperFont, text: &str) -> (i32, i32) {
    let v_metrics = font.main.v_metrics(scale);
    let (mut w, mut h) = (0, 0);

    for g in font.layout(text, scale, rusttype::point(0.0, v_metrics.ascent)) {
        if let Some(bb) = g.pixel_bounding_box() {
            w = std::cmp::max(w, bb.max.x);
            h = std::cmp::max(h, bb.max.y);
        }
    }
    (w, h)
}

pub fn text_width(scale: rusttype::Scale, font: &SuperFont, text: &str) -> i32 {
    let mut w = 0;
    for g in font.layout(text, scale, rusttype::point(0.0, 0.0)) {
        if let Some(bb) = g.pixel_bounding_box() {
            w = std::cmp::max(w, bb.max.x);
        }
    }
    w
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
/// Returns the size of the text in pixels.
///
/// assumes that emojis were parsed out
pub fn parsed_text_size_with_emojis(
    scale: rusttype::Scale,
    font: &SuperFont,
    text: &str,
) -> (i32, i32) {
    let v_metrics = font.main.v_metrics(scale);

    let (mut w, mut h) = (0, 0);

    for g in font.layout_with_emojis(
        text,
        &[],
        &mut 0,
        scale,
        rusttype::point(0.0, v_metrics.ascent),
    ) {
        if let Some(bb) = g.0.pixel_bounding_box() {
            w = std::cmp::max(w, bb.max.x);
            h = std::cmp::max(h, bb.max.y);
        }
    }
    (w, h)
}

#[cfg(feature = "emoji")]
pub fn text_size_with_emojis(scale: rusttype::Scale, font: &SuperFont, text: &str) -> (i32, i32) {
    parsed_text_size_with_emojis(scale, font, &crate::emoji::parse::clean_emojis(text))
}

#[cfg(feature = "emoji")]
/// Returns the width of the text in pixels.
///
/// assumes that emojis were parsed out
pub fn parsed_text_width_with_emojis(scale: rusttype::Scale, font: &SuperFont, text: &str) -> i32 {
    let mut w = 0;
    for g in font.layout_with_emojis(&text, &[], &mut 0, scale, rusttype::point(0.0, 0.0)) {
        if let Some(bb) = g.0.pixel_bounding_box() {
            w = std::cmp::max(w, bb.max.x);
        }
    }
    w
}

#[cfg(feature = "emoji")]
pub fn text_width_with_emojis(scale: rusttype::Scale, font: &SuperFont, text: &str) -> i32 {
    parsed_text_width_with_emojis(scale, font, &crate::emoji::parse::clean_emojis(text))
}

#[cfg(feature = "emoji")]
/// Returns the size of the text in pixels.
///
/// assumes that emojis were parsed out
pub fn parsed_text_size_multiline_with_emojis(
    lines: &Vec<String>,
    font: &SuperFont,
    scale: rusttype::Scale,
    line_spacing: f32,
) -> (i32, i32) {
    let mut width = 0;

    for line in lines {
        width = width.max(parsed_text_size_with_emojis(scale, font, line).0);
    }

    (
        width,
        ((lines.len() as f32 * scale.y * line_spacing) - (line_spacing - 1.0) * scale.y).round()
            as i32,
    )
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
