use std::sync::Arc;

pub struct SuperFont<'f> {
    pub(crate) main: rusttype::Font<'f>,
    pub(crate) fallbacks: Arc<Vec<rusttype::Font<'f>>>,

    #[cfg(feature = "emoji")]
    pub emoji_options: crate::emoji::EmojiOptions,
}

impl<'f> SuperFont<'f> {
    pub fn new(font: rusttype::Font<'f>, fallbacks: Vec<rusttype::Font<'f>>) -> SuperFont<'f> {
        Self {
            main: font,
            fallbacks: Arc::new(fallbacks),
            #[cfg(feature = "emoji")]
            emoji_options: crate::emoji::EmojiOptions::default(),
        }
    }

    pub fn main(&self) -> &rusttype::Font<'f> {
        &self.main
    }

    pub fn fallbacks(&self) -> &[rusttype::Font<'f>] {
        &self.fallbacks
    }

    #[cfg(feature = "emoji")]
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
    emojis: &'s [crate::emoji::source::EmojiType],
    emoji_index: usize,
    emoji_scale: rusttype::Scale,
    caret: f32,
    scale: rusttype::Scale,
    start: rusttype::Point<f32>,
    last_glyph: Option<rusttype::GlyphId>,
}

#[cfg(feature = "emoji")]
impl<'a, 'font, 's> Iterator for SuperEmojiLayoutIter<'a, 'font, 's> {
    type Item = (
        rusttype::PositionedGlyph<'font>,
        Option<&'s crate::emoji::source::EmojiType>,
    );

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
                if c == crate::emoji::parse::PLACEHOLDER {
                    if let Some(emoji) = self.emojis.get(self.emoji_index) {
                        let g = crate::emoji::EMOJI_FONT.glyph(crate::emoji::parse::PLACEHOLDER);
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

    #[cfg(feature = "emoji")]
    pub fn layout_with_emojis<'a, 's>(
        &'a self,
        text: &'s str,
        emojis: &'s [crate::emoji::source::EmojiType],
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
