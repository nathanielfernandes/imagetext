use tiny_skia::*;

use crate::{
    measure::text_width, outliner::TextAlign, prelude::WrapStyle, render::render_text_fn,
    superfont::SuperFont, wrap::text_wrap,
};

use super::outline::Outline;

pub fn draw_text_mut(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    text: &str,
) -> Result<(), &'static str> {
    render_text_fn(image, fill, outline, |td| {
        td.draw_text(text, x, y, font, scale);
    })
}

pub fn draw_text_anchored(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    text: &str,
) -> Result<(), &'static str> {
    render_text_fn(image, fill, outline, |td| {
        td.draw_text_anchored(text, x, y, ax, ay, font, scale);
    })
}

pub fn draw_text_multiline(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
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
) -> Result<(), &'static str> {
    render_text_fn(image, fill, outline, |td| {
        td.draw_text_multiline(lines, x, y, ax, ay, width, font, scale, line_spacing, align);
    })
}

pub fn draw_text_wrapped(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
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
) -> Result<(), &'static str> {
    render_text_fn(image, fill, outline, |td| {
        let lines = text_wrap(text, width as i32, font, scale, wrap_style, text_width);
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
    })
}

#[cfg(feature = "emoji")]
use crate::emoji::source::EmojiResolver;

#[cfg(feature = "emoji")]
pub fn draw_parsed_text_mut_with_emojis<R: EmojiResolver>(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    emoji_resolver: R,
    text: &str, // assmes parsed
    emojis: &[crate::emoji::source::EmojiType],
    emoji_idx: &mut usize,
) -> Result<(), &'static str> {
    crate::render::render_text_emoji_fn(
        image,
        fill,
        outline,
        font,
        emoji_resolver,
        emojis.len(),
        |td, acc| {
            td.draw_text_with_emojis(text, emojis, emoji_idx, x, y, font, scale, acc);
        },
    )
}

#[cfg(feature = "emoji")]
pub fn draw_text_mut_with_emojis<R: EmojiResolver>(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    emoji_resolver: R,
    text: &str, // assmes parsed
) -> Result<(), &'static str> {
    let (text, emojis) = crate::emoji::parse::parse_out_emojis(
        text,
        font.emoji_options.parse_shortcodes,
        font.emoji_options.parse_discord_emojis,
    );

    draw_parsed_text_mut_with_emojis(
        image,
        fill,
        outline,
        x,
        y,
        scale,
        font,
        emoji_resolver,
        &text,
        &emojis,
        &mut 0,
    )
}

#[cfg(feature = "emoji")]
pub fn draw_parsed_text_anchored_with_emojis<R: EmojiResolver>(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    emoji_resolver: R,
    text: &str,
    emojis: &[crate::emoji::source::EmojiType],
    emoji_idx: &mut usize,
) -> Result<(), &'static str> {
    crate::render::render_text_emoji_fn(
        image,
        fill,
        outline,
        font,
        emoji_resolver,
        emojis.len(),
        |td, acc| {
            td.draw_text_anchored_with_emojis(
                text, emojis, emoji_idx, x, y, ax, ay, font, scale, acc,
            );
        },
    )
}

#[cfg(feature = "emoji")]
pub fn draw_text_anchored_with_emojis<R: EmojiResolver>(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    emoji_resolver: R,
    text: &str,
) -> Result<(), &'static str> {
    let (text, emojis) = crate::emoji::parse::parse_out_emojis(
        text,
        font.emoji_options.parse_shortcodes,
        font.emoji_options.parse_discord_emojis,
    );

    draw_parsed_text_anchored_with_emojis(
        image,
        fill,
        outline,
        x,
        y,
        ax,
        ay,
        scale,
        font,
        emoji_resolver,
        &text,
        &emojis,
        &mut 0,
    )
}

#[cfg(feature = "emoji")]
pub fn draw_parsed_text_multiline_with_emojis<R: EmojiResolver>(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    width: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    emoji_resolver: R,
    lines: &Vec<String>,
    emojis: &[crate::emoji::source::EmojiType],
    emoji_idx: &mut usize,
    line_spacing: f32,
    align: TextAlign,
) -> Result<(), &'static str> {
    crate::render::render_text_emoji_fn(
        image,
        fill,
        outline,
        font,
        emoji_resolver,
        emojis.len(),
        |td, acc| {
            td.draw_text_multiline_with_emojis(
                lines,
                emojis,
                emoji_idx,
                x,
                y,
                ax,
                ay,
                width,
                font,
                scale,
                line_spacing,
                align,
                acc,
            );
        },
    )
}

#[cfg(feature = "emoji")]
pub fn draw_text_multiline_with_emojis<R: EmojiResolver>(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    width: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    emoji_resolver: R,
    lines: &Vec<String>,
    line_spacing: f32,
    align: TextAlign,
) -> Result<(), &'static str> {
    let mut emojis = Vec::new();

    let lines = lines
        .iter()
        .map(|l| {
            let (text, line_emojis) = crate::emoji::parse::parse_out_emojis(
                l,
                font.emoji_options.parse_shortcodes,
                font.emoji_options.parse_discord_emojis,
            );

            emojis.extend(line_emojis);

            text
        })
        .collect();

    draw_parsed_text_multiline_with_emojis(
        image,
        fill,
        outline,
        x,
        y,
        ax,
        ay,
        width,
        scale,
        font,
        emoji_resolver,
        &lines,
        &emojis,
        &mut 0,
        line_spacing,
        align,
    )
}

#[cfg(feature = "emoji")]
pub fn draw_parsed_text_wrapped_with_emojis<R: EmojiResolver>(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    width: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    emoji_resolver: R,
    text: &str,
    emojis: &[crate::emoji::source::EmojiType],
    emoji_idx: &mut usize,
    line_spacing: f32,
    align: TextAlign,
    wrap_style: WrapStyle,
) -> Result<(), &'static str> {
    crate::render::render_text_emoji_fn(
        image,
        fill,
        outline,
        font,
        emoji_resolver,
        emojis.len(),
        |td, acc| {
            let lines = text_wrap(
                text,
                width as i32,
                font,
                scale,
                wrap_style,
                crate::measure::parsed_text_width_with_emojis,
            );
            td.draw_text_multiline_with_emojis(
                &lines,
                emojis,
                emoji_idx,
                x,
                y,
                ax,
                ay,
                width,
                font,
                scale,
                line_spacing,
                align,
                acc,
            );
        },
    )
}

#[cfg(feature = "emoji")]
pub fn draw_text_wrapped_with_emojis(
    image: &mut image::RgbaImage,
    fill: &Paint,
    outline: Outline,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    width: f32,
    scale: rusttype::Scale,
    font: &SuperFont,
    emoji_resolver: impl EmojiResolver,
    text: &str,
    line_spacing: f32,
    align: TextAlign,
    wrap_style: WrapStyle,
) -> Result<(), &'static str> {
    let (text, emojis) = crate::emoji::parse::parse_out_emojis(
        text,
        font.emoji_options.parse_shortcodes,
        font.emoji_options.parse_discord_emojis,
    );

    draw_parsed_text_wrapped_with_emojis(
        image,
        fill,
        outline,
        x,
        y,
        ax,
        ay,
        width,
        scale,
        font,
        emoji_resolver,
        &text,
        &emojis,
        &mut 0,
        line_spacing,
        align,
        wrap_style,
    )
}
