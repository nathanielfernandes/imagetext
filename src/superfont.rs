pub struct SuperFont<'a, 'f> {
    pub inner: &'a rusttype::Font<'f>,
    pub fallbacks: &'a [rusttype::Font<'f>],
}

impl<'a, 'f> SuperFont<'a, 'f> {
    pub fn new(
        font: &'a rusttype::Font<'f>,
        fallbacks: &'a [rusttype::Font<'f>],
    ) -> SuperFont<'a, 'f> {
        Self {
            inner: font,
            fallbacks,
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

    fn next(&mut self) -> Option<rusttype::PositionedGlyph<'font>> {
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

impl<'superfont, 'f> SuperFont<'superfont, 'f> {
    pub fn layout<'a, 's>(
        &'a self,
        text: &'s str,
        scale: rusttype::Scale,
        start: rusttype::Point<f32>,
    ) -> SuperLayoutIter<'a, 'superfont, 's> {
        SuperLayoutIter {
            font: self.inner,
            fallbacks: self.fallbacks,
            chars: text.chars(),
            caret: 0.0,
            scale,
            start,
            last_glyph: None,
        }
    }
}
