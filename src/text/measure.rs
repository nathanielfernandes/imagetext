use crate::prelude::SuperFont;

use super::parse::{Token, Tokens};

pub(crate) fn tokens_width(scale: rusttype::Scale, font: &SuperFont, tokens: &Vec<Token>) -> i32 {
    let mut w = 0;

    for g in font.single_line_layout(tokens.chars().iter(), scale, rusttype::point(0.0, 0.0)) {
        if let Some(bb) = g.pixel_bounding_box() {
            w = std::cmp::max(w, bb.max.x);
        }
    }

    w
}
