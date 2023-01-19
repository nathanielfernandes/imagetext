use imagetext::prelude::*;

fn main() {
    // replace this with your own font
    let coolvetica = load_font("./coolvetica.ttf").unwrap();
    let emoji_fallback = load_font("./notob.ttf").unwrap();
    let jp_fallback = load_font("./notojp.otf").unwrap();

    let fallbacks = [emoji_fallback, jp_fallback];
    let font = SuperFont::new(&coolvetica, &fallbacks);

    let mut image = image::RgbaImage::from_pixel(512, 512, image::Rgba([255, 255, 255, 255]));

    let rainbow_fill = rainbow(point(0.0, 0.0), point(256.0, 256.0));

    let text = "hello my 😓 n🐢ame i☕s 会のすべ aての構成員 nathan and i drink soup boop coop, the quick brown fox";

    draw_text_wrapped(
        &mut image,
        &BLACK,
        Some(&stroke(2.0)),
        Some(&rainbow_fill),
        256.0,
        256.0,
        0.5,
        0.5,
        512.0,
        scale(67.0),
        &font,
        text,
        1.0,
        TextAlign::Center,
    )
    .unwrap();

    image.save("image.png").unwrap();
}
