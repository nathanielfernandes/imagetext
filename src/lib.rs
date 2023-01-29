pub mod drawing;
pub mod layout;
pub mod measure;
pub mod outliner;
pub mod superfont;
pub mod wrap;

pub mod prelude {
    pub use crate::drawing::paint::*;
    pub use crate::drawing::text::*;
    pub use crate::drawing::utils::*;
    pub use crate::measure::*;
    pub use crate::wrap::*;

    pub use crate::outliner::TextAlign;
    pub use crate::superfont::*;

    pub use rusttype::{Font, Scale};
    pub use tiny_skia::{Color, GradientStop, LinearGradient, Paint, Pixmap, RadialGradient};

    #[cfg(feature = "emoji")]
    pub use crate::emoji::{source::*, EmojiOptions};

    #[cfg(feature = "default-resolver")]
    pub use crate::emoji::default_resolver::DefaultEmojiResolver;
}

#[cfg(feature = "emoji")]
pub mod emoji;

#[cfg(feature = "fontdb")]
pub mod fontdb;
