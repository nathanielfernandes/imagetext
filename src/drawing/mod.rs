pub mod paint;
pub mod text;
pub mod utils;

pub mod prelude {
    pub use crate::drawing::paint::*;
    pub use crate::drawing::text::*;
    pub use crate::drawing::utils::*;

    pub use crate::outliner::TextAlign;
    pub use crate::superfont::{SuperFont, SuperLayoutIter};

    pub use rusttype::{Font, Scale};
    pub use tiny_skia::{Color, Paint, Pixmap};
}
