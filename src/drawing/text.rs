use tiny_skia::*;

use crate::{
    measure::text_size,
    outliner::{TextAlign, TextDrawer},
    prelude::WrapStyle,
    superfont::SuperFont,
    wrap::text_wrap,
};

use super::{paint::BLACK, utils::pixmap_mut};

pub fn draw_text_mut(
    image: &mut image::RgbaImage,
    fill: &Paint,
    stroke: Option<&Stroke>,
    stroke_fill: Option<&Paint>,
    x: f32,
    y: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    text: &str,
) -> Result<(), String> {
    match pixmap_mut(image) {
        Some(mut pixmap) => {
            let path = {
                let mut pb = PathBuilder::new();
                let mut td = TextDrawer::new(&mut pb);

                td.draw_text(text, x, y, font, scale);

                if pb.is_empty() {
                    None
                } else {
                    Some(pb.finish().ok_or("Failed to build text path.")?)
                }
            };

            if let Some(path) = path {
                if let Some(stroke) = stroke {
                    let stroke_fill = stroke_fill.unwrap_or(&BLACK);
                    pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
                }

                pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
            }
            Ok(())
        }
        None => Err("Could not create pixmap".to_string()),
    }
}

pub fn draw_text_anchored(
    image: &mut image::RgbaImage,
    fill: &Paint,
    stroke: Option<&Stroke>,
    stroke_fill: Option<&Paint>,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    text: &str,
) -> Result<(), String> {
    match pixmap_mut(image) {
        Some(mut pixmap) => {
            let path = {
                let mut pb = PathBuilder::new();
                let mut td = TextDrawer::new(&mut pb);

                td.draw_text_anchored(text, x, y, ax, ay, font, scale);

                if pb.is_empty() {
                    None
                } else {
                    Some(pb.finish().ok_or("Failed to build text path.")?)
                }
            };

            if let Some(path) = path {
                if let Some(stroke) = stroke {
                    let stroke_fill = stroke_fill.unwrap_or(&BLACK);
                    pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
                }

                pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
            }
            Ok(())
        }
        None => Err("Could not create pixmap".to_string()),
    }
}

pub fn draw_text_multiline(
    image: &mut image::RgbaImage,
    fill: &Paint,
    stroke: Option<&Stroke>,
    stroke_fill: Option<&Paint>,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    width: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    lines: &Vec<String>,
    line_spacing: f32,
    align: TextAlign,
) -> Result<(), String> {
    match pixmap_mut(image) {
        Some(mut pixmap) => {
            let path = {
                let mut pb = PathBuilder::new();
                let mut td = TextDrawer::new(&mut pb);

                td.draw_text_multiline(
                    lines,
                    x,
                    y,
                    ax,
                    ay,
                    width,
                    font,
                    scale,
                    line_spacing,
                    align,
                );

                if pb.is_empty() {
                    None
                } else {
                    Some(pb.finish().ok_or("Failed to build text path.")?)
                }
            };

            if let Some(path) = path {
                if let Some(stroke) = stroke {
                    let stroke_fill = stroke_fill.unwrap_or(&BLACK);
                    pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
                }

                pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
            }
            Ok(())
        }
        None => Err("Could not create pixmap".to_string()),
    }
}

pub fn draw_text_wrapped(
    image: &mut image::RgbaImage,
    fill: &Paint,
    stroke: Option<&Stroke>,
    stroke_fill: Option<&Paint>,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    width: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    text: &str,
    line_spacing: f32,
    align: TextAlign,
    wrap_style: WrapStyle,
) -> Result<(), String> {
    match pixmap_mut(image) {
        Some(mut pixmap) => {
            let path = {
                let mut pb = PathBuilder::new();
                let mut td = TextDrawer::new(&mut pb);

                let lines = text_wrap(text, width as i32, font, scale, wrap_style, text_size);
                td.draw_text_multiline(
                    &lines,
                    x,
                    y,
                    ax,
                    ay,
                    width,
                    font,
                    scale,
                    line_spacing,
                    align,
                );
                if pb.is_empty() {
                    None
                } else {
                    Some(pb.finish().ok_or("Failed to build text path.")?)
                }
            };

            if let Some(path) = path {
                if let Some(stroke) = stroke {
                    let stroke_fill = stroke_fill.unwrap_or(&BLACK);
                    pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
                }

                pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
            }

            Ok(())
        }
        None => Err("Could not create pixmap".to_string()),
    }
}

// #[cfg(feature = "emoji")]
// use crate::emoji::source::EmojiResolver;

// #[cfg(feature = "emoji")]
// fn resolve_emoji_ims(
//     td: &mut TextDrawer,
//     font: &SuperFont,
//     emoji_resolver: &mut impl crate::emoji::source::EmojiResolver,
// ) -> Vec<(image::RgbaImage, (i64, i64))> {
//     let emojis = td.emojis();
//     let unresolved: Vec<crate::emoji::source::UnresolvedEmoji> = emojis
//         .iter()
//         .enumerate()
//         .map(|(id, emoji)| crate::emoji::source::UnresolvedEmoji {
//             id,
//             path: font.emoji_options.path_for(&emoji.emoji),
//             size: emoji.size,
//         })
//         .collect();

//     emoji_resolver
//         .resolve(&unresolved)
//         .into_iter()
//         .filter_map(|resolved| {
//             if let Some(image) = resolved.image {
//                 let (w, h) = image.dimensions();
//                 Some((
//                     image,
//                     (
//                         emojis[resolved.id].position.0 - w as i64 / 2,
//                         emojis[resolved.id].position.1 - h as i64 / 2,
//                     ),
//                 ))
//             } else {
//                 td.draw_glyph(&emojis[resolved.id].fallback);
//                 None
//             }
//         })
//         .collect()
// }

// #[cfg(feature = "emoji")]
// pub fn draw_text_mut_with_emojis<R: EmojiResolver>(
//     image: &mut image::RgbaImage,
//     fill: &Paint,
//     stroke: Option<&Stroke>,
//     stroke_fill: Option<&Paint>,
//     x: f32,
//     y: f32,
//     scale: rusttype::Scale,
//     font: &SuperFont,
//     mut emoji_resolver: R,
//     text: &str,
// ) -> Result<(), String> {
//     match pixmap_mut(image) {
//         Some(mut pixmap) => {
//             let mut pb = PathBuilder::new();
//             let mut td = TextDrawer::new(&mut pb);

//             td.draw_text_with_emojis(text, x, y, font, scale);

//             let emoji_ims = resolve_emoji_ims(&mut td, font, &mut emoji_resolver);

//             let path = if pb.is_empty() {
//                 None
//             } else {
//                 Some(pb.finish().ok_or("Failed to build text path.")?)
//             };

//             if let Some(path) = path {
//                 if let Some(stroke) = stroke {
//                     let stroke_fill = stroke_fill.unwrap_or(&BLACK);
//                     pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
//                 }

//                 pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
//             }

//             for (im, (x, y)) in emoji_ims {
//                 image::imageops::overlay(image, &im, x, y);
//             }

//             Ok(())
//         }
//         None => Err("Could not create pixmap".to_string())?,
//     }
// }

// #[cfg(feature = "emoji")]
// pub fn draw_text_anchored_with_emojis<R: EmojiResolver>(
//     image: &mut image::RgbaImage,
//     fill: &Paint,
//     stroke: Option<&Stroke>,
//     stroke_fill: Option<&Paint>,
//     x: f32,
//     y: f32,
//     ax: f32,
//     ay: f32,
//     scale: rusttype::Scale,
//     font: &SuperFont,
//     mut emoji_resolver: R,
//     text: &str,
// ) -> Result<(), String> {
//     match pixmap_mut(image) {
//         Some(mut pixmap) => {
//             let mut pb = PathBuilder::new();
//             let mut td = TextDrawer::new(&mut pb);

//             td.draw_text_anchored_with_emojis(text, x, y, ax, ay, font, scale);

//             let emoji_ims = resolve_emoji_ims(&mut td, font, &mut emoji_resolver);

//             let path = if pb.is_empty() {
//                 None
//             } else {
//                 Some(pb.finish().ok_or("Failed to build text path.")?)
//             };

//             if let Some(path) = path {
//                 if let Some(stroke) = stroke {
//                     let stroke_fill = stroke_fill.unwrap_or(&BLACK);
//                     pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
//                 }

//                 pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
//             }

//             for (im, (x, y)) in emoji_ims {
//                 image::imageops::overlay(image, &im, x, y);
//             }

//             Ok(())
//         }
//         None => Err("Could not create pixmap".to_string())?,
//     }
// }

// #[cfg(feature = "emoji")]
// pub fn draw_text_multiline_with_emojis<R: EmojiResolver>(
//     image: &mut image::RgbaImage,
//     fill: &Paint,
//     stroke: Option<&Stroke>,
//     stroke_fill: Option<&Paint>,
//     x: f32,
//     y: f32,
//     ax: f32,
//     ay: f32,
//     width: f32,
//     scale: rusttype::Scale,
//     font: &SuperFont,
//     mut emoji_resolver: R,
//     lines: &Vec<String>,
//     line_spacing: f32,
//     align: TextAlign,
// ) -> Result<(), String> {
//     match pixmap_mut(image) {
//         Some(mut pixmap) => {
//             let mut pb = PathBuilder::new();
//             let mut td = TextDrawer::new(&mut pb);

//             td.draw_text_multiline_with_emojis(
//                 &lines,
//                 x,
//                 y,
//                 ax,
//                 ay,
//                 width,
//                 font,
//                 scale,
//                 line_spacing,
//                 align,
//             );

//             let emoji_ims = resolve_emoji_ims(&mut td, font, &mut emoji_resolver);

//             let path = if pb.is_empty() {
//                 None
//             } else {
//                 Some(pb.finish().ok_or("Failed to build text path.")?)
//             };

//             if let Some(path) = path {
//                 if let Some(stroke) = stroke {
//                     let stroke_fill = stroke_fill.unwrap_or(&BLACK);
//                     pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
//                 }

//                 pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
//             }

//             for (im, (x, y)) in emoji_ims {
//                 image::imageops::overlay(image, &im, x, y);
//             }

//             Ok(())
//         }
//         None => Err("Could not create pixmap".to_string())?,
//     }
// }

// #[cfg(feature = "emoji")]
// pub fn draw_text_wrapped_with_emojis<R: EmojiResolver>(
//     image: &mut image::RgbaImage,
//     fill: &Paint,
//     stroke: Option<&Stroke>,
//     stroke_fill: Option<&Paint>,
//     x: f32,
//     y: f32,
//     ax: f32,
//     ay: f32,
//     width: f32,
//     scale: rusttype::Scale,
//     font: &SuperFont,
//     mut emoji_resolver: R,
//     text: &str,
//     line_spacing: f32,
//     align: TextAlign,
//     wrap_style: WrapStyle,
// ) -> Result<(), String> {
//     use crate::measure::text_size_with_emojis;

//     match pixmap_mut(image) {
//         Some(mut pixmap) => {
//             let mut pb = PathBuilder::new();
//             let mut td = TextDrawer::new(&mut pb);

//             let lines = text_wrap(
//                 text,
//                 width as i32,
//                 font,
//                 scale,
//                 wrap_style,
//                 text_size_with_emojis,
//             );
//             td.draw_text_multiline_with_emojis(
//                 &lines,
//                 x,
//                 y,
//                 ax,
//                 ay,
//                 width,
//                 font,
//                 scale,
//                 line_spacing,
//                 align,
//             );

//             let emoji_ims = resolve_emoji_ims(&mut td, font, &mut emoji_resolver);

//             let path = if pb.is_empty() {
//                 None
//             } else {
//                 Some(pb.finish().ok_or("Failed to build text path.")?)
//             };

//             if let Some(path) = path {
//                 if let Some(stroke) = stroke {
//                     let stroke_fill = stroke_fill.unwrap_or(&BLACK);
//                     pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
//                 }

//                 pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
//             }

//             for (im, (x, y)) in emoji_ims {
//                 image::imageops::overlay(image, &im, x, y);
//             }

//             Ok(())
//         }
//         None => Err("Could not create pixmap".to_string())?,
//     }
// }
