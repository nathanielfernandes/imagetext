use fxhash::FxHashMap;
use image::RgbaImage;
use include_dir::{include_dir, Dir};
use once_cell::sync::{Lazy, OnceCell};
use regex::{Captures, Regex};

pub(crate) static EMOJIS: Lazy<Vec<&'static emojis::Emoji>> = Lazy::new(|| {
    let mut emojis = emojis::iter().collect::<Vec<_>>();
    let mut emojis_tones = Vec::new();
    for emoji in emojis.iter() {
        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::Light) {
            emojis_tones.push(e);
        }

        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::MediumLight) {
            emojis_tones.push(e);
        }

        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::Medium) {
            emojis_tones.push(e);
        }

        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::MediumDark) {
            emojis_tones.push(e);
        }

        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::Dark) {
            emojis_tones.push(e);
        }
    }
    emojis.extend(emojis_tones);

    emojis
});

static EMOJI_RE: Lazy<Regex> = Lazy::new(|| {
    let mut emojis = EMOJIS
        .iter()
        .map(|e| format!("{}", regex::escape(e.as_str())))
        .collect::<Vec<_>>();

    emojis.sort_by(|a, b| b.len().cmp(&a.len()));
    Regex::new(&emojis.join("|")).expect("Failed to compile emoji regex")
});

static EMOJI_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/assets/twemojis");

// Noto Emoji font (used for calculating emoji sizes / positions)
pub(crate) static EMOJI_FONT: Lazy<rusttype::Font<'static>> = Lazy::new(|| {
    let font_data = include_bytes!("assets/NotoEmoji-Bold.ttf");
    rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Failed to load emoji font")
});

pub fn load_static() {
    let _ = *EMOJIS;
    let _ = *EMOJI_RE;
    let _ = *EMOJI_FONT;
}

fn emoji_path(emoji: &str) -> String {
    let mut e = String::new();
    for c in {
        if emoji.contains('\u{200d}') {
            emoji.to_string()
        } else {
            emoji.replace("\u{fe0f}", "")
        }
    }
    .chars()
    {
        e.push_str(&format!("{:x}", c as u32));
        e.push('-');
    }
    e.pop();

    e.push_str(".png");

    e
}

pub(crate) static EMOJI_IMAGES: Lazy<FxHashMap<&'static str, OnceCell<RgbaImage>>> =
    Lazy::new(|| {
        let mut map = FxHashMap::default();

        for emoji in EMOJIS.iter() {
            if EMOJI_DIR.contains(emoji_path(emoji.as_str())) {
                map.insert(emoji.as_str(), OnceCell::new());
            }
        }

        map
    });

pub(crate) fn get_emoji_image(emoji: &str) -> Option<&'static RgbaImage> {
    if let Some(cell) = EMOJI_IMAGES.get(emoji) {
        if let Some(img) = cell.get() {
            return Some(img);
        }

        if let Some(path) = EMOJI_DIR.get_file(emoji_path(emoji)) {
            let img = image::load_from_memory(path.contents()).expect("Failed to load emoji emoji");
            return Some(cell.get_or_init(|| img.to_rgba8()));
        }
    }

    None
}

pub(crate) fn get_emoji_sized(emoji: &str, size: u32) -> Option<RgbaImage> {
    if let Some(img) = get_emoji_image(emoji) {
        Some(image::imageops::resize(
            img,
            size,
            size,
            image::imageops::FilterType::Lanczos3,
        ))
    } else {
        None
    }
}

pub(crate) const PLACEHOLDER: char = '😀';

pub(crate) fn parse_out_emojis(text: &str) -> (String, Vec<&'static emojis::Emoji>) {
    let mut emojis: Vec<&'static emojis::Emoji> = Vec::new();
    let s = EMOJI_RE
        .replace_all(text, |caps: &Captures| {
            if let Some(m) = caps.get(0) {
                let s = m.as_str();
                if let Some(e) = emojis::get(s) {
                    emojis.push(e);
                    return PLACEHOLDER.to_string();
                }
            }
            "".to_string()
        })
        .to_string();

    (s.replace("\u{fe0f}", ""), emojis)
}

#[test]
fn parse_out_emojis_test() {
    let text = "h😨😰 my 😓 n🐢ame i☕s 会のすべ aての構成員 nathan and i drin😳🥵 boop coop, the quick brown fox jumps over the lazy dog";

    let (s, emojis) = parse_out_emojis(text);

    assert_eq!(s, "h😀😀 my 😀 n😀ame i😀s 会のすべ aての構成員 nathan and i drin😀😀 boop coop, the quick brown fox jumps over the lazy dog");
    assert_eq!(
        emojis,
        vec![
            emojis::get("😨").unwrap(),
            emojis::get("😰").unwrap(),
            emojis::get("😓").unwrap(),
            emojis::get("🐢").unwrap(),
            emojis::get("☕").unwrap(),
            emojis::get("😳").unwrap(),
            emojis::get("🥵").unwrap(),
        ]
    );
}
