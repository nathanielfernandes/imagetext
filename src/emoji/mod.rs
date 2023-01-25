use once_cell::sync::Lazy;

use self::source::{EmojiPath, EmojiSource, EmojiType};

pub mod parse;
pub mod source;

#[cfg(feature = "default-resolver")]
pub mod default_resolver;

// Noto Emoji font (used for calculating emoji sizes / positions)
pub(crate) static EMOJI_FONT: Lazy<rusttype::Font<'static>> = Lazy::new(|| {
    let font_data = include_bytes!("../assets/NotoEmoji-Bold.ttf");
    rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Failed to load emoji font")
});

#[derive(Debug, Clone, Copy)]
pub struct EmojiOptions<'a> {
    pub scale: f32,
    pub shift: (i64, i64),

    pub allow_shortcodes: bool,
    pub allow_discord: bool,
    pub source: EmojiSource<'a>,
}

impl Default for EmojiOptions<'_> {
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

impl EmojiOptions<'_> {
    pub fn discord() -> Self {
        Self {
            allow_discord: true,
            ..Self::default()
        }
    }

    pub fn path_for(&self, emoji: &EmojiType) -> EmojiPath {
        self.source.build_path(emoji, self.allow_discord)
    }
}

impl<'a> EmojiOptions<'a> {
    pub fn from_source(source: EmojiSource<'a>) -> EmojiOptions<'a> {
        Self {
            source,
            ..Default::default()
        }
    }

    pub fn dir(dir: &'a str) -> EmojiOptions<'a> {
        Self::from_source(EmojiSource::Dir(dir))
    }
}
