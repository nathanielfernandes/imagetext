use std::sync::Arc;

use crate::text::{
    layout::SingleLineLayoutIter,
    parse::{EmojiType, Token, PLACEHOLDER_EMOJI},
};

pub struct SuperFont<'f> {
    pub(crate) main: rusttype::Font<'f>,
    pub(crate) fallbacks: Arc<Vec<rusttype::Font<'f>>>,

    pub emoji_options: crate::emoji::EmojiOptions,
}

impl<'f> SuperFont<'f> {
    pub fn new(font: rusttype::Font<'f>, fallbacks: Vec<rusttype::Font<'f>>) -> SuperFont<'f> {
        Self {
            main: font,
            fallbacks: Arc::new(fallbacks),
            emoji_options: crate::emoji::EmojiOptions::default(),
        }
    }

    pub fn main(&self) -> &rusttype::Font<'f> {
        &self.main
    }

    pub fn fallbacks(&self) -> &[rusttype::Font<'f>] {
        &self.fallbacks
    }

    pub fn with_emoji_options(
        font: rusttype::Font<'f>,
        fallbacks: Vec<rusttype::Font<'f>>,
        emoji_options: crate::emoji::EmojiOptions,
    ) -> SuperFont<'f> {
        Self {
            main: font,
            fallbacks: Arc::new(fallbacks),
            emoji_options,
        }
    }
}

pub struct SuperLayoutIter<'a, 'font, 's> {
    font: &'a rusttype::Font<'font>,
    fallbacks: &'a [rusttype::Font<'font>],

    chars: core::str::Chars<'s>,

    caret: f32,
    scale: rusttype::Scale,
    start: rusttype::Point<f32>,
    last_glyph: Option<rusttype::GlyphId>,
}

impl<'a, 'font, 's> Iterator for SuperLayoutIter<'a, 'font, 's> {
    type Item = rusttype::PositionedGlyph<'font>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|c| {
            let g = self.font.glyph(c);
            let id = g.id();

            if id.0 == 0 {
                for font in self.fallbacks {
                    let g = font.glyph(c);
                    let id = g.id();

                    if id.0 == 0 {
                        continue;
                    }

                    let g = g.scaled(self.scale);

                    if let Some(last) = self.last_glyph {
                        self.caret += font.pair_kerning(self.scale, last, id);
                    }

                    let advance_width = g.h_metrics().advance_width;

                    let g = g.positioned(rusttype::point(self.start.x + self.caret, self.start.y));

                    self.caret += advance_width;
                    self.last_glyph = Some(id);

                    return g;
                }
            }

            let g = g.scaled(self.scale);

            if let Some(last) = self.last_glyph {
                self.caret += self.font.pair_kerning(self.scale, last, id);
            }

            let advance_width = g.h_metrics().advance_width;

            let g = g.positioned(rusttype::point(self.start.x + self.caret, self.start.y));

            self.caret += advance_width;
            self.last_glyph = Some(id);

            g
        })
    }
}

#[cfg(feature = "emoji")]
pub struct SuperEmojiLayoutIter<'a, 'font, 's> {
    font: &'a rusttype::Font<'font>,
    fallbacks: &'a [rusttype::Font<'font>],
    chars: core::str::Chars<'s>,
    emojis: &'s [EmojiType],
    emoji_index: usize,
    emoji_scale: rusttype::Scale,
    caret: f32,
    scale: rusttype::Scale,
    start: rusttype::Point<f32>,
    last_glyph: Option<rusttype::GlyphId>,
}

#[cfg(feature = "emoji")]
impl<'a, 'font, 's> Iterator for SuperEmojiLayoutIter<'a, 'font, 's> {
    type Item = (rusttype::PositionedGlyph<'font>, Option<&'s EmojiType>);

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|c| {
            let g = self.font.glyph(c);
            let id = g.id();

            if id.0 == 0 {
                for font in self.fallbacks {
                    let g = font.glyph(c);
                    let id = g.id();

                    if id.0 == 0 {
                        continue;
                    }

                    let g = g.scaled(self.scale);

                    if let Some(last) = self.last_glyph {
                        self.caret += font.pair_kerning(self.scale, last, id);
                    }

                    let advance_width = g.h_metrics().advance_width;

                    let g = g.positioned(rusttype::point(self.start.x + self.caret, self.start.y));

                    self.caret += advance_width;
                    self.last_glyph = Some(id);

                    return (g, None);
                }

                // If we get here, we didn't find a fallback font that had the glyph.
                if c == PLACEHOLDER_EMOJI {
                    if let Some(emoji) = self.emojis.get(self.emoji_index) {
                        let g = crate::emoji::EMOJI_FONT.glyph(PLACEHOLDER_EMOJI);
                        let g = g.scaled(self.emoji_scale);
                        let id = g.id();

                        if let Some(last) = self.last_glyph {
                            self.caret +=
                                crate::emoji::EMOJI_FONT.pair_kerning(self.scale, last, id);
                        }

                        let advance_width = g.h_metrics().advance_width;

                        let g =
                            g.positioned(rusttype::point(self.start.x + self.caret, self.start.y));

                        self.caret += advance_width;
                        self.last_glyph = Some(id);

                        self.emoji_index += 1;

                        return (g, Some(emoji));
                    }
                }
            }

            let g = g.scaled(self.scale);

            if let Some(last) = self.last_glyph {
                self.caret += self.font.pair_kerning(self.scale, last, id);
            }

            let advance_width = g.h_metrics().advance_width;

            let g = g.positioned(rusttype::point(self.start.x + self.caret, self.start.y));

            self.caret += advance_width;
            self.last_glyph = Some(id);

            (g, None)
        })
    }
}

impl<'f> SuperFont<'f> {
    pub fn layout<'a, 's>(
        &'a self,
        text: &'s str,
        scale: rusttype::Scale,
        start: rusttype::Point<f32>,
    ) -> SuperLayoutIter<'a, 'f, 's> {
        SuperLayoutIter {
            font: &self.main,
            fallbacks: &self.fallbacks,
            chars: text.chars(),
            caret: 0.0,
            scale,
            start,
            last_glyph: None,
        }
    }

    pub fn single_line_layout<'iter, 't, T: Iterator<Item = &'t Token>>(
        &'iter self,
        tokens: T,
        scale: rusttype::Scale,
        start: rusttype::Point<f32>,
    ) -> SingleLineLayoutIter<'iter, 'f, 't, T> {
        SingleLineLayoutIter::new(self, tokens, scale, start)
    }

    #[cfg(feature = "emoji")]
    pub fn layout_with_emojis<'a, 's>(
        &'a self,
        text: &'s str,
        emojis: &'s [EmojiType],
        emoji_scale: f32,
        scale: rusttype::Scale,
        start: rusttype::Point<f32>,
    ) -> SuperEmojiLayoutIter<'a, 'f, 's> {
        SuperEmojiLayoutIter {
            font: &self.main,
            fallbacks: &self.fallbacks,
            chars: text.chars(),
            emojis,
            emoji_index: 0,
            emoji_scale: rusttype::Scale {
                x: scale.x * emoji_scale,
                y: scale.y * emoji_scale,
            },
            caret: 0.0,
            scale,
            start,
            last_glyph: None,
        }
    }
}
