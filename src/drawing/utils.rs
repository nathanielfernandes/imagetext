use tiny_skia::*;

pub fn point(x: f32, y: f32) -> Point {
    Point::from_xy(x, y)
}

pub fn stroke(width: f32) -> Stroke {
    Stroke {
        width,
        line_cap: LineCap::Butt,
        line_join: LineJoin::Round,
        ..Stroke::default()
    }
}

pub fn scale(size: f32) -> rusttype::Scale {
    rusttype::Scale::uniform(size)
}

pub fn load_font(path: &str) -> Result<rusttype::Font<'static>, String> {
    let data = std::fs::read(path).map_err(|e| e.to_string())?;
    let font = rusttype::Font::try_from_vec(data).ok_or("Could not load font")?;
    Ok(font)
}

// does not copy the image
pub fn pixmap_mut<'a>(image: &'a mut image::RgbaImage) -> Option<PixmapMut<'a>> {
    let (w, h) = image.dimensions();
    PixmapMut::from_bytes(image, w, h)
}
