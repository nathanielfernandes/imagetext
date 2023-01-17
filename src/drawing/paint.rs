use tiny_skia::*;

pub const BLACK: Paint<'static> = Paint {
    shader: Shader::SolidColor(Color::BLACK),
    blend_mode: BlendMode::SourceOver,
    anti_alias: true,
    force_hq_pipeline: false,
};

pub const WHITE: Paint<'static> = Paint {
    shader: Shader::SolidColor(Color::WHITE),
    blend_mode: BlendMode::SourceOver,
    anti_alias: true,
    force_hq_pipeline: false,
};

pub fn rainbow(start: Point, end: Point) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.shader = LinearGradient::new(
        start,
        end,
        vec![
            GradientStop::new(0.0, Color::from_rgba8(255, 0, 0, 255)),
            GradientStop::new(0.15, Color::from_rgba8(255, 255, 0, 255)),
            GradientStop::new(0.33, Color::from_rgba8(0, 255, 0, 255)),
            GradientStop::new(0.49, Color::from_rgba8(0, 255, 255, 255)),
            GradientStop::new(0.67, Color::from_rgba8(0, 0, 255, 255)),
            GradientStop::new(0.84, Color::from_rgba8(255, 0, 255, 255)),
            GradientStop::new(1.0, Color::from_rgba8(255, 0, 0, 255)),
        ],
        SpreadMode::Repeat,
        Transform::default(),
    )
    .unwrap();
    paint
}

pub fn ez_gradient(start: Point, end: Point, colors: Vec<Color>) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.shader = LinearGradient::new(
        start,
        end,
        colors
            .iter()
            .enumerate()
            .map(|(i, c)| GradientStop::new(i as f32 / (colors.len() - 1) as f32, *c))
            .collect(),
        SpreadMode::Repeat,
        Transform::default(),
    )
    .unwrap();
    paint
}

pub fn paint_from_rgba_slice(slice: &[u8; 4]) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.set_color_rgba8(slice[0], slice[1], slice[2], slice[3]);
    paint
}

pub fn paint_from_rgba(r: u8, g: u8, b: u8, a: u8) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.set_color_rgba8(r, g, b, a);
    paint
}

pub fn paint_from_rgb(r: u8, g: u8, b: u8) -> Paint<'static> {
    paint_from_rgba(r, g, b, 255)
}

pub fn black() -> Paint<'static> {
    paint_from_rgb(0, 0, 0)
}

pub fn white() -> Paint<'static> {
    paint_from_rgb(255, 255, 255)
}

pub fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba8(r, g, b, a)
}

pub fn color_from_rgba_slice(slice: &[u8; 4]) -> Color {
    Color::from_rgba8(slice[0], slice[1], slice[2], slice[3])
}

pub fn color_rgb(r: u8, g: u8, b: u8) -> Color {
    color(r, g, b, 255)
}
