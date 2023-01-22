use tiny_skia::*;

use crate::{
    outliner::{TextAlign, TextDrawer},
    superfont::SuperFont,
    wrap::word_wrap,
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

                pb.finish().ok_or("Could not create path")?
            };

            if let Some(stroke) = stroke {
                let stroke_fill = stroke_fill.unwrap_or(&BLACK);
                pixmap.stroke_path(&path, stroke_fill, stroke, Transform::identity(), None);
            }

            pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
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

                pb.finish().ok_or("Could not create path")?
            };

            if let Some(stroke) = stroke {
                let stroke_fill = stroke_fill.unwrap_or(&BLACK);
                pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
            }

            pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
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

                pb.finish().ok_or("Could not create path")?
            };

            if let Some(stroke) = stroke {
                let stroke_fill = stroke_fill.unwrap_or(&BLACK);
                pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
            }

            pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
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
) -> Result<(), String> {
    match pixmap_mut(image) {
        Some(mut pixmap) => {
            #[cfg(not(feature = "emoji"))]
            let path = {
                let mut pb = PathBuilder::new();
                let mut td = TextDrawer::new(&mut pb);

                let lines = word_wrap(text, width as i32, font, scale);
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

            #[cfg(feature = "emoji")]
            let (path, emojis) = {
                let mut pb = PathBuilder::new();
                let mut td = TextDrawer::new(&mut pb);

                let lines = word_wrap(text, width as i32, font, scale);

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

                let emojis = td.emojis();

                if pb.is_empty() {
                    (None, emojis)
                } else {
                    (
                        Some(pb.finish().ok_or("Failed to build text path.")?),
                        emojis,
                    )
                }
            };

            if let Some(path) = path {
                if let Some(stroke) = stroke {
                    let stroke_fill = stroke_fill.unwrap_or(&BLACK);
                    pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
                }

                pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
            }

            #[cfg(feature = "emoji")]
            for emoji in emojis {
                if let Some(im) = crate::emoji::get_emoji_sized(emoji.emoji.as_str(), emoji.size) {
                    image::imageops::overlay(image, &im, emoji.position.0, emoji.position.1);
                }
            }

            Ok(())
        }
        None => Err("Could not create pixmap".to_string()),
    }
}
