use tiny_skia::PathBuilder;

use crate::{measure::text_size, superfont::SuperFont};

#[derive(Debug, Clone, Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[cfg(feature = "emoji")]
#[derive(Debug, Clone)]
pub struct PositionedEmoji<'a> {
    pub position: (i64, i64),
    pub size: u32,
    pub emoji: crate::emoji::source::EmojiType,

    pub fallback: rusttype::PositionedGlyph<'a>,
}

pub struct TextDrawer<'a> {
    pub pb: &'a mut PathBuilder,
    offset: rusttype::Point<f32>,
}
impl<'a> TextDrawer<'a> {
    pub fn new(pb: &'a mut PathBuilder) -> Self {
        Self {
            pb,
            offset: rusttype::Point { x: 0.0, y: 0.0 },
        }
    }

    #[inline]
    pub fn draw_glyph(&mut self, glyph: &rusttype::PositionedGlyph<'_>) {
        self.offset = glyph.position();
        glyph.unpositioned().build_outline(self);
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        font: &SuperFont,
        scale: rusttype::Scale,
    ) {
        let v_metrics = font.main.v_metrics(scale);
        for g in font.layout(text, scale, rusttype::point(x, y + v_metrics.ascent)) {
            if let Some(_) = g.pixel_bounding_box() {
                self.draw_glyph(&g);
            }
        }
    }

    pub fn draw_text_anchored(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        ax: f32,
        ay: f32,
        font: &SuperFont,
        scale: rusttype::Scale,
    ) {
        let (w, h) = text_size(scale, font, text);
        let x = x - w as f32 * ax;
        let y = y - h as f32 * ay;

        self.draw_text(text, x, y, font, scale);
    }

    pub fn draw_text_multiline(
        &mut self,
        lines: &Vec<String>,
        x: f32,
        y: f32,
        ax: f32,
        ay: f32,
        width: f32,
        font: &SuperFont,
        scale: rusttype::Scale,
        line_spacing: f32,
        align: TextAlign,
    ) {
        let h = (lines.len() as f32 * (scale.y * line_spacing)) - (line_spacing - 1.0) * scale.y;

        let mut x = x - width * ax;
        let mut y = y - h * ay;

        let ax = match align {
            TextAlign::Left => 0.0,
            TextAlign::Center => {
                x += width * 0.5;
                0.5
            }
            TextAlign::Right => {
                x += width;
                1.0
            }
        };

        for line in lines {
            self.draw_text_anchored(line, x, y, ax, 0.0, font, scale);
            y += scale.y * line_spacing;
        }
    }
}
#[cfg(feature = "emoji")]
impl<'a> TextDrawer<'a> {
    pub fn draw_text_with_emojis<'f>(
        &mut self,
        text: &str, // assumes that text is parsed
        emojis: &[crate::emoji::source::EmojiType],
        emoji_idx: &mut usize,
        x: f32,
        y: f32,
        font: &'f SuperFont,
        scale: rusttype::Scale,

        emoji_acc: &mut Vec<PositionedEmoji<'f>>,
    ) {
        let v_metrics = font.main.v_metrics(scale);
        for (g, emoji) in font.layout_with_emojis(
            text,
            emojis,
            emoji_idx,
            scale,
            rusttype::point(x, y + v_metrics.ascent),
        ) {
            if let Some(bb) = g.pixel_bounding_box() {
                match emoji {
                    Some(emoji) => {
                        let w = bb.width();
                        let w2 = (w as f32 / 2.0).round() as i32;

                        let position = (bb.min.x + w2, bb.min.y + w2);

                        emoji_acc.push(PositionedEmoji {
                            position: (
                                position.0 as i64 + font.emoji_options.shift.0,
                                position.1 as i64 + font.emoji_options.shift.1,
                            ),
                            size: w as u32,
                            emoji: *emoji,

                            fallback: g,
                        });
                    }
                    None => {
                        self.draw_glyph(&g);
                    }
                }
            }
        }
    }

    pub fn draw_text_anchored_with_emojis<'f>(
        &mut self,
        text: &str, // assumes that text is parsed
        emojis: &[crate::emoji::source::EmojiType],
        emoji_idx: &mut usize,
        x: f32,
        y: f32,
        ax: f32,
        ay: f32,
        font: &'f SuperFont,
        scale: rusttype::Scale,

        emoji_acc: &mut Vec<PositionedEmoji<'f>>,
    ) {
        let (w, h) = crate::measure::parsed_text_size_with_emojis(scale, font, text);
        let x = x - w as f32 * ax;
        let y = y - h as f32 * ay;

        self.draw_text_with_emojis(text, emojis, emoji_idx, x, y, font, scale, emoji_acc);
    }

    pub fn draw_text_multiline_with_emojis<'f>(
        &mut self,
        lines: &Vec<String>,
        emojis: &[crate::emoji::source::EmojiType],
        emoji_idx: &mut usize,
        x: f32,
        y: f32,
        ax: f32,
        ay: f32,
        width: f32,
        font: &'f SuperFont,
        scale: rusttype::Scale,
        line_spacing: f32,
        align: TextAlign,

        emoji_acc: &mut Vec<PositionedEmoji<'f>>,
    ) {
        let h = (lines.len() as f32 * (scale.y * line_spacing)) - (line_spacing - 1.0) * scale.y;

        let mut x = x - width * ax;
        let mut y = y - h * ay;

        let ax = match align {
            TextAlign::Left => 0.0,
            TextAlign::Center => {
                x += width * 0.5;
                0.5
            }
            TextAlign::Right => {
                x += width;
                1.0
            }
        };

        for line in lines {
            self.draw_text_anchored_with_emojis(
                line, emojis, emoji_idx, x, y, ax, 0.0, font, scale, emoji_acc,
            );
            y += scale.y * line_spacing;
        }
    }
}

impl rusttype::OutlineBuilder for TextDrawer<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.pb.move_to(x + self.offset.x, y + self.offset.y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.pb.line_to(x + self.offset.x, y + self.offset.y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.pb.quad_to(
            x1 + self.offset.x,
            y1 + self.offset.y,
            x + self.offset.x,
            y + self.offset.y,
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.pb.cubic_to(
            x1 + self.offset.x,
            y1 + self.offset.y,
            x2 + self.offset.x,
            y2 + self.offset.y,
            x + self.offset.x,
            y + self.offset.y,
        );
    }

    fn close(&mut self) {
        self.pb.close();
    }
}
