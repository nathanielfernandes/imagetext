use once_cell::sync::Lazy;

use crate::text::parse::EmojiType;

use self::source::{EmojiPath, EmojiSource};

pub mod source;

#[cfg(feature = "default-resolver")]
pub mod default_resolver;

// Noto Emoji font (used for calculating emoji sizes / positions)
pub(crate) static EMOJI_FONT: Lazy<rusttype::Font<'static>> = Lazy::new(|| {
    let font_data = include_bytes!("../assets/NotoEmoji-Bold.ttf");
    rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Failed to load emoji font")
});

#[derive(Debug, Clone)]
pub struct EmojiOptions {
    pub scale: f32,
    pub shift: (i64, i64),

    pub parse_shortcodes: bool,
    pub parse_discord_emojis: bool,
    pub source: EmojiSource,
}

impl Default for EmojiOptions {
    fn default() -> Self {
        Self {
            scale: 1.0,
            shift: (0, 0),

            parse_shortcodes: true,
            parse_discord_emojis: false,
            source: EmojiSource::Twitter,
        }
    }
}

impl EmojiOptions {
    pub fn discord() -> Self {
        Self {
            parse_discord_emojis: true,
            ..Self::default()
        }
    }

    pub fn path_for(&self, emoji: &EmojiType) -> EmojiPath {
        self.source.build_path(emoji, self.parse_discord_emojis)
    }
}

impl EmojiOptions {
    pub fn from_source(source: EmojiSource) -> EmojiOptions {
        Self {
            source,
            ..Default::default()
        }
    }

    pub fn dir<S: Into<String>>(dir: S) -> EmojiOptions {
        Self::from_source(EmojiSource::Dir(dir.into()))
    }
}
