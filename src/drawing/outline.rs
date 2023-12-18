use crate::prelude::BLACK;

use tiny_skia::{Paint, Stroke};

#[derive(Debug, Clone)]
pub enum Outline<'a> {
    Solid {
        stroke: &'a Stroke,
        fill: &'a Paint<'a>,
    },
    // Soft {
    //     radius: u8,
    //     color: [u8; 4],

    //     // if true, it will assume that the image to be outlined is empty
    //     // and will not create a duplicate buffer of it (which is slower)
    //     no_dup: bool,
    // },
    None,
}

impl<'a> Outline<'a> {
    pub fn solid(stroke: &'a Stroke, fill: Option<&'a Paint<'a>>) -> Self {
        Outline::Solid {
            stroke,
            fill: fill.unwrap_or(&BLACK),
        }
    }

    // pub fn soft(radius: u8, color: [u8; 4]) -> Self {
    //     Outline::Soft {
    //         radius,
    //         color,
    //         no_dup: false,
    //     }
    // }

    // pub fn soft_nd(radius: u8, color: [u8; 4]) -> Self {
    //     Outline::Soft {
    //         radius,
    //         color,
    //         no_dup: true,
    //     }
    // }
}
