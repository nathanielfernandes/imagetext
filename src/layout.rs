use crate::prelude::SuperFont;

impl<'font> SuperFont<'font> {
    pub fn layout<'a, 's>(
        &'a self,
        text: &'s str,
        scale: rusttype::Scale,
        start: rusttype::Point<f32>,
    ) -> LayoutIter<'a, 'font, 's> {
        LayoutIter {
            font: self,
            scale,
            start,
            caret: 0.0,
            last_glyph: None,
            chars: text.chars(),
        }
    }

    #[cfg(feature = "emoji")]
    pub fn layout_with_emojis<'a, 's>(
        &'a self,
        text: &'s str,
        emojis: &'s [crate::emoji::source::EmojiType],
        emoji_idx: &'s mut usize,
        scale: rusttype::Scale,
        start: rusttype::Point<f32>,
    ) -> LayoutWithEmojisIter<'a, 'font, 's> {
        LayoutWithEmojisIter {
            font: self,
            scale,
            emoji_scale: rusttype::Scale {
                x: self.emoji_options.scale * scale.x,
                y: self.emoji_options.scale * scale.y,
            },
            start,
            caret: 0.0,
            last_glyph: None,
            chars: text.chars(),
            emojis,
            emoji_idx,
        }
    }
}

pub struct LayoutIter<'iter, 'font, 'text> {
    font: &'iter SuperFont<'font>,
    chars: core::str::Chars<'text>,

    caret: f32,
    scale: rusttype::Scale,
    start: rusttype::Point<f32>,

    last_glyph: Option<rusttype::GlyphId>,
}

impl<'iter, 'font, 'text> Iterator for LayoutIter<'iter, 'font, 'text> {
    type Item = rusttype::PositionedGlyph<'font>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.chars.next() {
            let g = self.font.main.glyph(c);
            let id = g.id();

            if id.0 == 0 {
                for font in self.font.fallbacks.iter() {
                    let g = font.glyph(c);
                    let id = g.id();

                    if id.0 != 0 {
                        continue;
                    }

                    let g = g.scaled(self.scale);

                    if let Some(last) = self.last_glyph {
                        self.caret += font.pair_kerning(self.scale, last, id);
                    }

                    let advance_width = g.h_metrics().advance_width;
                    let g = g.positioned(rusttype::point(self.caret + self.start.x, self.start.y));

                    self.caret += advance_width;
                    self.last_glyph = Some(id);

                    return Some(g);
                }
            }
            let g = g.scaled(self.scale);

            if let Some(last) = self.last_glyph {
                self.caret += self.font.main.pair_kerning(self.scale, last, id);
            }

            let advance_width = g.h_metrics().advance_width;

            let g = g.positioned(rusttype::point(self.caret + self.start.x, self.start.y));

            self.caret += advance_width;
            self.last_glyph = Some(id);

            Some(g)
        } else {
            None
        }
    }
}

#[cfg(feature = "emoji")]
pub struct LayoutWithEmojisIter<'iter, 'font, 'text> {
    font: &'iter SuperFont<'font>,
    chars: core::str::Chars<'text>,

    caret: f32,
    scale: rusttype::Scale,
    emoji_scale: rusttype::Scale,

    start: rusttype::Point<f32>,

    last_glyph: Option<rusttype::GlyphId>,

    emojis: &'text [crate::emoji::source::EmojiType],
    emoji_idx: &'text mut usize,
}

#[cfg(feature = "emoji")]
impl<'iter, 'font, 'text> Iterator for LayoutWithEmojisIter<'iter, 'font, 'text> {
    type Item = (
        rusttype::PositionedGlyph<'font>,
        Option<&'text crate::emoji::source::EmojiType>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.chars.next() {
            if c == crate::emoji::parse::PLACEHOLDER_EMOJI {
                let g = crate::emoji::EMOJI_FONT.glyph(crate::emoji::parse::PLACEHOLDER_EMOJI);
                let id = g.id();

                let g = g.scaled(self.emoji_scale);

                if let Some(last) = self.last_glyph {
                    self.caret += crate::emoji::EMOJI_FONT.pair_kerning(self.scale, last, id);
                }

                let advance_width = g.h_metrics().advance_width;
                let g = g.positioned(rusttype::point(self.caret + self.start.x, self.start.y));

                self.caret += advance_width;
                self.last_glyph = Some(id);

                let emoji = self.emojis.get(*self.emoji_idx);
                *self.emoji_idx += 1;

                return Some((g, emoji));
            }

            let g = self.font.main.glyph(c);
            let id = g.id();

            if id.0 == 0 {
                for font in self.font.fallbacks.iter() {
                    let g = font.glyph(c);
                    let id = g.id();

                    if id.0 != 0 {
                        continue;
                    }

                    let g = g.scaled(self.scale);

                    if let Some(last) = self.last_glyph {
                        self.caret += font.pair_kerning(self.scale, last, id);
                    }

                    let advance_width = g.h_metrics().advance_width;
                    let g = g.positioned(rusttype::point(self.caret + self.start.x, self.start.y));

                    self.caret += advance_width;
                    self.last_glyph = Some(id);

                    return Some((g, None));
                }
            }
            let g = g.scaled(self.scale);

            if let Some(last) = self.last_glyph {
                self.caret += self.font.main.pair_kerning(self.scale, last, id);
            }

            let advance_width = g.h_metrics().advance_width;

            let g = g.positioned(rusttype::point(self.caret + self.start.x, self.start.y));

            self.caret += advance_width;
            self.last_glyph = Some(id);

            Some((g, None))
        } else {
            None
        }
    }
}
