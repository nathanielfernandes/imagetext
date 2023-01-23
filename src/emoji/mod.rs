use once_cell::sync::Lazy;

use self::source::{EmojiPath, EmojiSource, EmojiType};

pub mod parse;
pub mod source;

#[cfg(feature = "default-emoji-resolver")]
pub mod default_resolver;

// Noto Emoji font (used for calculating emoji sizes / positions)
pub(crate) static EMOJI_FONT: Lazy<rusttype::Font<'static>> = Lazy::new(|| {
    let font_data = include_bytes!("../assets/NotoEmoji-Bold.ttf");
    rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Failed to load emoji font")
});

pub struct EmojiOptions {
    pub scale: f32,
    pub shift: (i64, i64),

    pub allow_shortcodes: bool,
    pub allow_discord: bool,
    pub source: EmojiSource,
}

impl Default for EmojiOptions {
    fn default() -> Self {
        Self {
            scale: 1.0,
            shift: (0, 0),

            allow_shortcodes: true,
            allow_discord: false,
            source: EmojiSource::Twitter,
        }
    }
}

impl EmojiOptions {
    pub fn path_for(&self, emoji: &EmojiType) -> EmojiPath {
        self.source.build_path(emoji, self.allow_discord)
    }
}
