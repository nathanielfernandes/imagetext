use once_cell::sync::Lazy;
use regex::Regex;

use super::source::EmojiType;

static EMOJIS: Lazy<Vec<&'static emojis::Emoji>> = Lazy::new(|| {
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
static EMOJI_SHORT_CODES_RE_STR: Lazy<String> = Lazy::new(|| {
    let mut emojis = EMOJIS
        .iter()
        .flat_map(|e| e.shortcodes())
        .map(|e| format!(":{}:", regex::escape(&e)))
        .collect::<Vec<_>>();

    emojis.sort_by(|a, b| b.len().cmp(&a.len()));
    emojis.join("|")
});
static DISCORD_EMOJI_RE_STR: &'static str = r"<a?:[a-zA-Z0-9_]{2, 32}:[0-9]{17,22}>";
static EMOJI_RE_STR: Lazy<String> = Lazy::new(|| {
    format!(
        "{}|{}|{}",
        EMOJI_UNICODE_RE_STR.as_str(),
        EMOJI_SHORT_CODES_RE_STR.as_str(),
        DISCORD_EMOJI_RE_STR
    )
});

static EMOJI_UNICODE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(&EMOJI_UNICODE_RE_STR).expect("Failed to compile emoji regex"));

static EMOJI_SHORT_CODES_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&EMOJI_SHORT_CODES_RE_STR).expect("Failed to compile emoji shortcode regex")
});

static DISCORD_EMOJI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<a?:[a-zA-Z0-9_]{2, 32}:([0-9]{17,22})>")
        .expect("Failed to compile discord emoji regex")
});

static TEXT_TOKEN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(r"{}|[\s\S]", EMOJI_RE_STR.as_str(),))
        .expect("Failed to compile emoji regex")
});

pub(crate) const PLACEHOLDER_EMOJI: char = '😀';

fn parse_unicode_emoji(s: &str) -> Option<EmojiType> {
    if EMOJI_UNICODE_RE.is_match(s) {
        if let Some(e) = emojis::get(s) {
            return Some(EmojiType::Regular(e));
        }
    }
    None
}

fn parse_discord_emoji(s: &str) -> Option<EmojiType> {
    if let Some(caps) = DISCORD_EMOJI_RE.captures(s) {
        if let Some(m) = caps.get(1) {
            if let Ok(id) = m.as_str().parse::<u64>() {
                return Some(EmojiType::Discord(id));
            }
        }
    }
    None
}

fn parse_shortcode_emoji(s: &str) -> Option<EmojiType> {
    if EMOJI_SHORT_CODES_RE.is_match(s) {
        if let Some(e) = emojis::get_by_shortcode(&s.replace(":", "")) {
            return Some(EmojiType::Regular(e));
        }
    }
    None
}

pub fn parse_out_emojis<'t>(
    text: &'t str,
    parse_shortcodes: bool,
    parse_discord_emojis: bool,
) -> (String, Vec<EmojiType>) {
    let mut parsed = String::with_capacity(text.len());
    let mut emojis = Vec::new();

    for caps in TEXT_TOKEN_RE.captures_iter(text) {
        if let Some(cap) = caps.get(0) {
            let s = cap.as_str();

            if let Some(emoji) = parse_unicode_emoji(s) {
                emojis.push(emoji);
                parsed.push(PLACEHOLDER_EMOJI);
                continue;
            }

            if parse_discord_emojis {
                if let Some(emoji) = parse_discord_emoji(s) {
                    emojis.push(emoji);
                    parsed.push(PLACEHOLDER_EMOJI);
                    continue;
                }
            }

            if parse_shortcodes {
                if let Some(emoji) = parse_shortcode_emoji(s) {
                    emojis.push(emoji);
                    parsed.push(PLACEHOLDER_EMOJI);
                    continue;
                }
            }

            parsed.push_str(s);
        }
    }

    (parsed, emojis)
}

pub fn clean_emojis(text: &str) -> String {
    let mut parsed = String::with_capacity(text.len());

    for caps in TEXT_TOKEN_RE.captures_iter(text) {
        if let Some(cap) = caps.get(0) {
            let s = cap.as_str();

            if let Some(_) = parse_unicode_emoji(s) {
                parsed.push(PLACEHOLDER_EMOJI);
                continue;
            }

            if let Some(_) = parse_discord_emoji(s) {
                parsed.push(PLACEHOLDER_EMOJI);
                continue;
            }

            if let Some(_) = parse_shortcode_emoji(s) {
                parsed.push(PLACEHOLDER_EMOJI);
                continue;
            }

            parsed.push_str(s);
        }
    }

    parsed
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
