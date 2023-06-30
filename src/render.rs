use tiny_skia::*;

use crate::{drawing::outline::Outline, outliner::TextDrawer, prelude::pixmap_mut};

#[inline]
pub(crate) fn with_pixmap(
    image: &mut image::RgbaImage,
    f: impl FnOnce(&mut PixmapMut),
) -> Result<(), &'static str> {
    let Some(mut pixmap) = pixmap_mut(image)  else {
        return Err("Could not create pixmap");
    };

    f(&mut pixmap);
    Ok(())
}

#[inline]
pub(crate) fn render_text_fn(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    f: impl FnOnce(&mut TextDrawer),
) -> Result<(), &'static str> {
    let path = {
        let mut pb = PathBuilder::new();
        let mut td = TextDrawer::new(&mut pb);

        f(&mut td);

        if pb.is_empty() {
            return Ok(());
        }

        pb.finish().ok_or("Failed to build text path.")?
    };

    render_path(image, &path, fill, outline)
}

#[inline]
pub(crate) fn render_path(
    image: &mut image::RgbaImage,
    path: &Path,
    fill: &Paint,
    outline: Outline,
) -> Result<(), &'static str> {
    match &outline {
        Outline::Solid {
            stroke,
            fill: stroke_fill,
        } => with_pixmap(image, |pixmap| {
            pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
            pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
        })?,
        Outline::None => with_pixmap(image, |pixmap| {
            pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None)
        })?,
    }

    Ok(())
}

#[cfg(feature = "emoji")]
pub(crate) fn resolve_emoji_ims(
    td: &mut TextDrawer,
    emojis: &Vec<crate::outliner::PositionedEmoji>,
    font: &crate::prelude::SuperFont,
    emoji_resolver: &mut impl crate::emoji::source::EmojiResolver,
) -> Vec<(image::RgbaImage, (i64, i64))> {
    let unresolved: Vec<crate::emoji::source::UnresolvedEmoji> = emojis
        .iter()
        .enumerate()
        .map(|(id, emoji)| crate::emoji::source::UnresolvedEmoji {
            id,
            path: font.emoji_options.path_for(&emoji.emoji),
            size: emoji.size,
        })
        .collect();

    emoji_resolver
        .resolve(&unresolved)
        .into_iter()
        .filter_map(|resolved| {
            if let Some(image) = resolved.image {
                let (w, h) = image.dimensions();
                Some((
                    image,
                    (
                        emojis[resolved.id].position.0 - w as i64 / 2,
                        emojis[resolved.id].position.1 - h as i64 / 2,
                    ),
                ))
            } else {
                td.draw_glyph(&emojis[resolved.id].fallback);
                None
            }
        })
        .collect()
}

#[cfg(feature = "emoji")]
use crate::emoji::source::EmojiResolver;

#[cfg(feature = "emoji")]
#[inline]
pub(crate) fn render_text_emoji_fn<'a, R: EmojiResolver>(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    font: &'a crate::prelude::SuperFont,
    mut emoji_resolver: R,
    emoji_count: usize,
    f: impl FnOnce(&mut TextDrawer, &mut Vec<crate::outliner::PositionedEmoji<'a>>),
) -> Result<(), &'static str> {
    let (path, emojis) = {
        let mut pb = PathBuilder::new();
        let mut td = TextDrawer::new(&mut pb);

        let mut emojis = Vec::with_capacity(emoji_count);
        f(&mut td, &mut emojis);

        let emojis = resolve_emoji_ims(&mut td, &emojis, &font, &mut emoji_resolver);

        if pb.is_empty() {
            return Ok(());
        }

        (pb.finish().ok_or("Failed to build text path.")?, emojis)
    };

    render_path_and_emojis(image, &path, emojis, fill, outline)
}

#[cfg(feature = "emoji")]
#[inline]
pub(crate) fn render_path_and_emojis(
    image: &mut image::RgbaImage,
    path: &Path,
    emojis: Vec<(image::RgbaImage, (i64, i64))>,
    fill: &Paint,
    outline: Outline,
) -> Result<(), &'static str> {
    match &outline {
        Outline::Solid {
            stroke,
            fill: stroke_fill,
        } => with_pixmap(image, |pixmap| {
            pixmap.stroke_path(&path, &stroke_fill, &stroke, Transform::identity(), None);
            pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
        })?,
        Outline::None => with_pixmap(image, |pixmap| {
            pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None)
        })?,
    }

    for (im, (x, y)) in emojis {
        image::imageops::overlay(image, &im, x, y);
    }

    Ok(())
}
