use crate::superfont::SuperFont;

#[derive(Debug, Clone, Copy)]
pub enum WrapStyle {
    Word,
    Character,
}

pub struct LineBreaker<'a, W> {
    words: W,
    width: i32,
    font: &'a SuperFont<'a>,
    scale: rusttype::Scale,

    current: Option<String>,
    chars_mode: bool,

    width_fn: fn(rusttype::Scale, &SuperFont, &str) -> i32,
}

impl<'a, W> LineBreaker<'a, W> {
    pub fn new<S>(
        words: W,
        width: i32,
        font: &'a SuperFont<'a>,
        scale: rusttype::Scale,
        chars_mode: bool,
        width_fn: fn(rusttype::Scale, &SuperFont, &str) -> i32,
    ) -> Self
    where
        W: Iterator<Item = S>,
        S: AsRef<str>,
    {
        {
            Self {
                words,
                width,
                font,
                scale,
                current: None,
                chars_mode,
                width_fn,
            }
        }
    }
}

impl<'a, W, S> Iterator for LineBreaker<'a, W>
where
    W: Iterator<Item = S>,
    S: AsRef<str>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.current.take().unwrap_or_else(String::new);

        while let Some(word) = self.words.next() {
            let word = word.as_ref();

            let mut new_line = line.clone();
            if !new_line.is_empty() && !self.chars_mode {
                new_line.push(' ');
            }
            new_line.push_str(word);

            let w = (self.width_fn)(self.scale, &self.font, &new_line);

            if w > self.width {
                if !line.is_empty() {
                    self.current = Some(word.to_string());
                    return Some(line);
                }
                line = word.to_string();
            } else {
                line = new_line;
            }
        }

        line = line.trim().to_string();

        if !line.is_empty() {
            Some(line)
        } else {
            None
        }
    }
}

pub trait Wrappable: Iterator {
    fn wrap_lines<'a>(
        self,
        width: i32,
        font: &'a SuperFont<'a>,
        scale: rusttype::Scale,
        chars_mode: bool,
        size_fn: fn(rusttype::Scale, &SuperFont, &str) -> i32,
    ) -> LineBreaker<'a, Self>
    where
        Self: Sized,
        Self::Item: AsRef<str>,
    {
        LineBreaker::new(self, width, font, scale, chars_mode, size_fn)
    }
}

impl<T, S> Wrappable for T where T: Iterator<Item = S> {}

pub fn text_wrap(
    text: &str,
    width: i32,
    font: &SuperFont,
    scale: rusttype::Scale,
    wrap_style: WrapStyle,
    width_fn: fn(rusttype::Scale, &SuperFont, &str) -> i32,
) -> Vec<String> {
    match wrap_style {
        WrapStyle::Word => text
            .split_whitespace()
            .wrap_lines(width, font, scale, false, width_fn)
            .collect(),
        WrapStyle::Character => {
            let mut result = Vec::new();
            for line in text
                .split_whitespace()
                .wrap_lines(width, font, scale, false, width_fn)
            {
                let w = (width_fn)(scale, font, &line);
                if w > width {
                    unicode_segmentation::UnicodeSegmentation::graphemes(line.as_str(), true)
                        .wrap_lines(width, font, scale, true, width_fn)
                        .for_each(|l| result.push(l));
                } else {
                    result.push(line);
                }
            }

            // final pass to clean up any character broken lines
            result
                .join(" ")
                .split_whitespace()
                .wrap_lines(width, font, scale, false, width_fn)
                .collect::<Vec<String>>()
        }
    }
}
