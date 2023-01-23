use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use super::source::EmojiType;

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

static EMOJI_UNICODE_RE_STR: Lazy<String> = Lazy::new(|| {
    let mut emojis = EMOJIS
        .iter()
        .map(|e| format!("{}", regex::escape(e.as_str())))
        .collect::<Vec<_>>();

    emojis.sort_by(|a, b| b.len().cmp(&a.len()));
    emojis.join("|")
});

static EMOJI_UNICODE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(&EMOJI_UNICODE_RE_STR).expect("Failed to compile emoji regex"));

static EMOJI_SHORT_CODES_RE_STR: Lazy<String> = Lazy::new(|| {
    let mut emojis = EMOJIS
        .iter()
        .flat_map(|e| e.shortcodes())
        .map(|e| format!(":{}:", regex::escape(&e)))
        .collect::<Vec<_>>();

    emojis.sort_by(|a, b| b.len().cmp(&a.len()));
    emojis.join("|")
});

static EMOJI_SHORT_CODES_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&EMOJI_SHORT_CODES_RE_STR).expect("Failed to compile emoji shortcode regex")
});

static DISCORD_EMOJI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<a?:[a-zA-Z0-9_]{2, 32}:([0-9]{17,22})>")
        .expect("Failed to compile discord emoji regex")
});

static EMOJI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        "{}|{}|{}",
        EMOJI_UNICODE_RE_STR.as_str(),
        EMOJI_SHORT_CODES_RE_STR.as_str(),
        r"<a?:[a-zA-Z0-9_]{2,32}:[0-9]{17,22}>"
    ))
    .expect("Failed to compile emoji regex")
});

pub(crate) const PLACEHOLDER: char = '😀';

pub(crate) fn parse_out_emojis(
    text: &str,
    shortcodes: bool,
    discord: bool,
) -> (String, Vec<EmojiType>) {
    let mut emojis: Vec<EmojiType> = Vec::new();

    let s = EMOJI_RE
        .replace_all(text, |caps: &Captures| {
            if let Some(m) = caps.get(0) {
                let s = m.as_str();

                if EMOJI_UNICODE_RE.is_match(s) {
                    if let Some(e) = emojis::get(s) {
                        emojis.push(EmojiType::Regular(e));
                        return PLACEHOLDER.to_string();
                    }
                }

                if discord {
                    if let Some(caps) = DISCORD_EMOJI_RE.captures(s) {
                        if let Some(m) = caps.get(1) {
                            let s = m.as_str();
                            if let Ok(id) = s.parse::<u64>() {
                                emojis.push(EmojiType::Discord(id));
                                return PLACEHOLDER.to_string();
                            }
                        }
                    }
                }

                if shortcodes && EMOJI_SHORT_CODES_RE.is_match(s) {
                    let s = s.replace(":", "");
                    if let Some(e) = emojis::get_by_shortcode(&s) {
                        emojis.push(EmojiType::Regular(e));
                        return PLACEHOLDER.to_string();
                    }
                }

                return s.to_string();
            }
            "".to_string()
        })
        .to_string();

    (s.replace("\u{fe0f}", ""), emojis)
}

#[test]
fn parse_out_emojis_test() {
    let text = "h😨😰 my 😓 n🐢ame i☕s 会のすべ aての構成員 nathan and i drin😳🥵 boop coop, the quick brown fox jumps over the lazy dog";
    let (s, emojis) = parse_out_emojis(text, false, false);

    assert_eq!(s, "h😀😀 my 😀 n😀ame i😀s 会のすべ aての構成員 nathan and i drin😀😀 boop coop, the quick brown fox jumps over the lazy dog");
    assert_eq!(
        emojis,
        vec![
            EmojiType::Regular(emojis::get("😨").unwrap()),
            EmojiType::Regular(emojis::get("😰").unwrap()),
            EmojiType::Regular(emojis::get("😓").unwrap()),
            EmojiType::Regular(emojis::get("🐢").unwrap()),
            EmojiType::Regular(emojis::get("☕").unwrap()),
            EmojiType::Regular(emojis::get("😳").unwrap()),
            EmojiType::Regular(emojis::get("🥵").unwrap()),
        ]
    );

    // shortcode test
    let text = "hello :smile: soup :pensive:";
    let (s, emojis) = parse_out_emojis(text, true, false);

    assert_eq!(s, "hello 😀 soup 😀");

    assert_eq!(
        emojis,
        vec![
            EmojiType::Regular(emojis::get_by_shortcode("smile").unwrap()),
            EmojiType::Regular(emojis::get_by_shortcode("pensive").unwrap()),
        ]
    );

    // discord test
    let text = "hello <a:smile:123456789012345678> soup :sob: <:soup:123456999012345678>";

    let (s, emojis) = parse_out_emojis(text, true, true);

    assert_eq!(s, "hello 😀 soup 😀 😀");

    assert_eq!(
        emojis,
        vec![
            EmojiType::Discord(123456789012345678),
            EmojiType::Regular(emojis::get_by_shortcode("sob").unwrap()),
            EmojiType::Discord(123456999012345678),
        ]
    );
}
