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
    emojis: &'s [&'static emojis::Emoji],
    emoji_index: usize,
    caret: f32,
    scale: rusttype::Scale,
    start: rusttype::Point<f32>,
    last_glyph: Option<rusttype::GlyphId>,
}

#[cfg(feature = "emoji")]
impl<'a, 'font, 's> Iterator for SuperEmojiLayoutIter<'a, 'font, 's> {
    type Item = (
        rusttype::PositionedGlyph<'font>,
        Option<&'static emojis::Emoji>,
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
                if c == crate::emoji::PLACEHOLDER {
                    if let Some(emoji) = self.emojis.get(self.emoji_index) {
                        let g = crate::emoji::EMOJI_FONT.glyph(crate::emoji::PLACEHOLDER);
                        let g = g.scaled(self.scale);
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

                        return (g, Some(*emoji));
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

impl<'superfont, 'f> SuperFont<'superfont, 'f> {
    #[cfg(not(feature = "emoji"))]
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

    #[cfg(feature = "emoji")]
    pub fn layout<'a, 's>(
        &'a self,
        text: &'s str,
        emojis: &'s [&'static emojis::Emoji],
        scale: rusttype::Scale,
        start: rusttype::Point<f32>,
    ) -> SuperEmojiLayoutIter<'a, 'superfont, 's> {
        SuperEmojiLayoutIter {
            font: self.inner,
            fallbacks: self.fallbacks,
            chars: text.chars(),
            emojis,
            emoji_index: 0,
            caret: 0.0,
            scale,
            start,
            last_glyph: None,
        }
    }
}

#[test]
fn soup() {
    use crate::{measure::text_size_multiline, prelude::*, wrap::word_wrap};

    // crate::emoji::load_static();

    let start = std::time::Instant::now();

    let coolvetica = load_font("./src/bin/coolvetica.ttf").unwrap();
    let emoji_fallback = load_font("./src/bin/notob.ttf").unwrap();
    let jp_fallback = load_font("./src/bin/notojp.otf").unwrap();

    let fallbacks = [jp_fallback];
    let font = SuperFont::new(&coolvetica, &fallbacks);

    let mut image = image::RgbaImage::from_pixel(512, 512, image::Rgba([255, 255, 255, 255]));

    let rainbow_fill = rainbow(point(0.0, 0.0), point(256.0, 256.0));

    let text = "😀 😃 😄 😁 😆 😅 😂 🤣 🥲 🥹 ☺️ 😊 😇 🙂 🙃 😉 😌 😍 🥰 😘 😗 😙 😚 😋 😛 😝 😜 🤪 🤨 🧐 🤓 😎 🥸 🤩 🥳 😏 😒 😞 😔 😟 😕 🙁 ☹️ 😣 😖 😫 😩 🥺 😢 😭 😮‍💨 😤 😠 😡 🤬 🤯 😳 🥵 🥶 😱 😨 😰 😥 😓 🫣 🤗 🫡 🤔 🫢 🤭 🤫 🤥 😶 😶‍🌫️ 😐 😑 😬 🫠 🙄 😯 😦 😧 😮 😲 🥱 😴 🤤 😪 😵 😵‍💫 🫥 🤐 🥴 🤢 🤮 🤧 😷 🤒 🤕 🤑 🤠 😈 👿 👹 👺 🤡 💩 👻 💀 ☠️ 👽 👾 🤖 🎃 😺 😸 😹 😻 😼 😽 🙀 😿 😾 👋 🤚 🖐 ✋ 🖖 👌 🤌 🤏 ✌️ 🤞 🫰 🤟 🤘 🤙 🫵 🫱 🫲 🫳 🫴 👈 👉 👆 🖕 👇 ☝️ 👍 👎 ✊ 👊 🤛 🤜 👏 🫶 🙌 👐 🤲 🤝 🙏 ✍️ 💅 🤳 💪 🦾 🦵 🦿 🦶 👣 👂 🦻 👃 🫀 🫁 🧠 🦷 🦴 👀 👁 👅 👄 🫦 💋 🩸 🧳 🌂 ☂️ 🧵 🪡 🪢 🧶 👓 🕶 🥽 🥼 🦺 👔 👕 👖 🧣 🧤 🧥 🧦 👗 👘 🥻 🩴 🩱 🩲 🩳 👙 👚 👛 👜 👝 🎒 👞 👟 🥾 🥿 👠 👡 🩰 👢 👑 👒 🎩 🎓 🧢 ⛑ 🪖 💄 💍 💼";

    // let lines = word_wrap(text, 512, &font, scale(67.0));
    // let (w, h) = text_size_multiline(&lines, &font, scale(67.0), 1.0);
    // println!("{}x{}", w, h);

    draw_text_wrapped(
        &mut image,
        &BLACK,
        Some(&stroke(2.0)),
        Some(&rainbow_fill),
        256.0,
        256.0,
        0.5,
        0.5,
        512.0,
        scale(25.0),
        &font,
        text,
        1.0,
        TextAlign::Left,
    )
    .unwrap();

    println!("Took {}ms", start.elapsed().as_millis());

    image.save("./src/bin/image.png").unwrap();
}
