use super::parse::{Token, PLACEHOLDER_EMOJI};
use crate::prelude::SuperFont;

pub struct SingleLineLayoutIter<'iter, 'font, 't, T: Iterator<Item = &'t Token>> {
    font: &'iter SuperFont<'font>,
    tokens: T,

    caret: f32,
    scale: rusttype::Scale,
    start: rusttype::Point<f32>,
    last_glyph: Option<rusttype::GlyphId>,
}

impl<'iter, 'font, 't, T> SingleLineLayoutIter<'iter, 'font, 't, T>
where
    T: Iterator<Item = &'t Token>,
{
    pub fn new(
        font: &'iter SuperFont<'font>,
        tokens: T,
        scale: rusttype::Scale,
        start: rusttype::Point<f32>,
    ) -> Self {
        Self {
            font,
            tokens,
            caret: 0.0,
            scale,
            start,
            last_glyph: None,
        }
    }

    fn positioned_glyph_for(&mut self, c: &char) -> rusttype::PositionedGlyph<'font> {
        let g = self.font.main.glyph(*c);
        let id = g.id();

        if id.0 == 0 {
            for font in self.font.fallbacks.iter() {
                let g = font.glyph(*c);
                let id = g.id();

                if id.0 == 0 {
                    continue;
                }

                let g = g.scaled(self.scale);

                if let Some(last) = self.last_glyph {
                    self.caret += font.pair_kerning(self.scale, last, id)
                }

                let advance_width = g.h_metrics().advance_width;

                let g = g.positioned(rusttype::Point {
                    x: self.start.x + self.caret,
                    y: self.start.y,
                });

                self.caret += advance_width;
                self.last_glyph = Some(id);

                return g;
            }
        }

        let g = g.scaled(self.scale);

        if let Some(last) = self.last_glyph {
            self.caret += self.font.main.pair_kerning(self.scale, last, id)
        }

        let advance_width = g.h_metrics().advance_width;

        let g = g.positioned(rusttype::Point {
            x: self.start.x + self.caret,
            y: self.start.y,
        });

        self.caret += advance_width;
        self.last_glyph = Some(id);

        g
    }
}

impl<'iter, 'font, 't, T> Iterator for SingleLineLayoutIter<'iter, 'font, 't, T>
where
    T: Iterator<Item = &'t Token>,
{
    type Item = rusttype::PositionedGlyph<'font>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.tokens.next() {
            match token {
                Token::Char(c) => Some(self.positioned_glyph_for(c)),
                Token::WhiteSpace => Some(self.positioned_glyph_for(&' ')),
                // just incase an emoji makes it through
                Token::Emoji(_) => Some(self.positioned_glyph_for(&PLACEHOLDER_EMOJI)),
                _ => None,
            }
        } else {
            None
        }
    }
}
