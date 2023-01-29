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
